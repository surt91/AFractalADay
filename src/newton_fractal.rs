extern crate std;
extern crate num;
extern crate png;
extern crate rayon;
extern crate rand;

use self::num::complex::Complex;

use std::path::Path;
use std::fs::File;
use std::io;
// To use encoder.set()
use self::png::HasParameters;
use itertools::Itertools;

use self::rayon::prelude::*;

pub struct NewtonFractal {
    pub a: f64,
    pub f: Box<Fn(Complex<f64>) -> Complex<f64> + Sync>,
    pub h: f64
}

struct Convergence {
    count: i64,
    value: Complex<f64>
}

fn hsv2rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    // https://de.wikipedia.org/wiki/HSV-Farbraum#Umrechnung_HSV_in_RGB

    let hi = (h * 6.).floor() as u32;
    let f = h * 6. - hi as f64;
    let p = v*(1.-s);
    let q = v*(1.-s*f);
    let t = v*(1.-s*(1.-f));

    match hi {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        6 => (v, t, p),
        _ => (0., 0., 0.)
    }
}

impl NewtonFractal {
    pub fn new(f: Box<Fn(Complex<f64>) -> Complex<f64> + Sync>) -> NewtonFractal {
        NewtonFractal {a: 1., f: f, h: 1e-4}
    }

    fn iterate(&self, mut state: Complex<f64>) -> Convergence {
        let mut ctr = 0;
        let threshold = 1e-4;
        let mut tmp;
        // *attention*: this is a do while loop, mind that the "body" is actually the
        // condition and the body is empty, thus omitted
        while {
            tmp = state;
            state = state - self.a * (self.f)(state) / self.fprime(state);
            ctr += 1;

            (state - tmp).norm() > threshold && ctr < 10000
        } {}
        Convergence {count: ctr, value: state}
    }

    fn fprime(&self, x: Complex<f64>) -> Complex<f64> {
        ((self.f)(x + self.h) - (self.f)(x - self.h)) / (2. * self.h)
    }

    pub fn random_formula() -> (Box<Fn(Complex<f64>) -> Complex<f64> + Sync>, String){
        // use up to 5 terms but at least 2
        let num_terms = (rand::random::<f64>() * 3.).floor() as i32 + 2;
        let mut terms: Vec<Box<Fn(Complex<f64>) -> Complex<f64> + Sync>> = Vec::new();
        let mut term_string: Vec<String> = Vec::new();

        let mut candidates: Vec<(Box<Fn(Complex<f64>) -> Complex<f64> + Sync>, String)> = Vec::new();

        let af = || 0.1f64.max((rand::random::<f64>() * 3. * 10.).round() / 10.);
        let mut a;
        let coeff = 2.;
        let b = (rand::random::<f64>() * 8.).floor();

        a = af();
        candidates.push((Box::new(move |_: Complex<f64>| Complex::new((a - 0.5) * 2. * coeff, 0.) ),
                         format!("{}", a)));
        a = af();
        candidates.push((Box::new(move |x: Complex<f64>| a * x),
                         format!("{} * x", a)));
        a = af();
        candidates.push((Box::new(move |x: Complex<f64>| a * x.powf(5.)),
                         format!("{} * x ^ 5", a)));
        a = af();
        candidates.push((Box::new(move |x: Complex<f64>| a * x.powf(6.)),
                         format!("{} * x ^ 6", a)));
        a = af();
        candidates.push((Box::new(move |x: Complex<f64>| a * x.powf(7.)),
                         format!("{} * x ^ 7", a)));
        a = af();
        candidates.push((Box::new(move |x: Complex<f64>| a * x.sin()),
                         format!("{} * sin(x)", a)));
        a = af();
        candidates.push((Box::new(move |x: Complex<f64>| a * x.cosh()),
                         format!("{} * cosh(x)", a)));
        a = af();
        candidates.push((Box::new(move |x: Complex<f64>| a * x.atanh()),
                         format!("{} * cosh(x)", a)));
        a = af();
        candidates.push((Box::new(move |x: Complex<f64>| a * (x+Complex {re: 0., im: 1.}).cosh()),
                         format!("{} * cosh(x+i)", a)));
        a = af();
        candidates.push((Box::new(move |x: Complex<f64>| a * (x*b.ln()).exp() ),
                         format!("{} * {} ^ x", a, b)));
        a = af();
        candidates.push((Box::new(move |x: Complex<f64>| a * x.exp() ),
                         format!("{} * exp(x)", a)));
        a = af();
        candidates.push((Box::new(move |x: Complex<f64>| a * x.ln() ),
                         format!("{} * ln(x)", a)));

        for _ in 0..num_terms {
            let num_cand = candidates.len();
            let neo = candidates.swap_remove((rand::random::<f64>() * num_cand as f64) as usize);
            terms.push(neo.0);
            term_string.push(neo.1);
        }

        let f = move |x| terms.iter()
                              .map(move |f| f(x))
                              .fold(Complex {re: 0., im: 0.}, |sum: Complex<f64>, x| sum + x);
        (Box::new(f), term_string.join(" + "))
    }

    fn raster(&self, x: i32, y: i32, xscale: f64, yscale: f64) -> Vec<Convergence> {
        let pixels: Vec<(i32, i32)> = iproduct!(0..y, 0..x).collect();
        pixels.par_iter()
              .map(|&(j, i)| {
                  let xp = (i-x/2) as f64 * xscale;
                  let yp = (j-y/2) as f64 * yscale;
                  let p = Complex {re: xp, im: yp};
                  let state = self.iterate(p);
                  state
              })
              .collect()
    }

    pub fn render(&self, resolution: (i32, i32), filename: &str) -> io::Result<i64> {
        let (x, y) = resolution;

        // use randomness to determine the colors
        let random_color = rand::random::<f64>();
        let random_count = rand::random::<f64>();
        let random_zoom = rand::random::<f64>();
        let scale = 4e-3 * random_zoom;

        let states = self.raster(x, y, scale, scale);
        let total_iterations: i64 = states.par_iter()
                                     .map(|i| i.count)
                                     .sum();
        println!("{:.2}M iterations", total_iterations as f64/1e6);

        let tmp_buffer: Vec<Vec<u8>> = states.par_iter()
                            .map(|i| {
                                let hue = (i.value.norm() * 10. * random_color) % 1.;
                                let saturation = 1f64;
                                let value = 1f64.min(i.count as f64 / (10. + 50. * random_count));

                                let (r, g, b) = hsv2rgb(hue, saturation, value);
                                let a = 255;

                                vec![(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8, a]
                            })
                            .collect();
        let buffer: Vec<u8> = tmp_buffer.into_iter()
                                        .flatten()
                                        .collect();

        let path = Path::new(filename);
        let file = File::create(path)?;
        let ref mut w = io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, x as u32, y as u32);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        writer.write_image_data(&buffer)?; // Save

        Ok(total_iterations)
    }
}
