extern crate std;
extern crate num;
extern crate png;
extern crate rayon;
extern crate rand;

use self::rand::Rng;
use self::num::complex::Complex;

use std::path::Path;
use std::fs::File;
use std::io;
// To use encoder.set()
use self::png::HasParameters;
use itertools::Itertools;

use self::rayon::prelude::*;

use functions::{Terms, Coef, Real, Cplx, ComplexFunction};
use color;

use style::Style;

pub struct IterationDetails {
    pub f: ComplexFunction,
    pub a: Coef,
    pub formula: String,
    pub prefix: String
}

pub struct Convergence {
    pub count: i64,
    pub value: Cplx
}

pub struct NewtonFractal {
    pub a: Coef,
    pub f: ComplexFunction,
    h: Real,
    rng: rand::StdRng,
    pub formula: String,
    style: Style
}

impl NewtonFractal {
    pub fn new(f: Option<ComplexFunction>, seed: Option<&[usize]>) -> NewtonFractal {
        let mut rng: rand::StdRng = match seed {
            Some(x) => rand::SeedableRng::from_seed(x),
            None => rand::StdRng::new().unwrap()
        };
        let formula = match f {
            Some(x) => IterationDetails {f: x, a: Coef::Real(1.), formula: "n/a".to_string(), prefix: "Newton Fractal of".to_string()},
            None => NewtonFractal::random_formula(&mut rng)
        };

        NewtonFractal {a: formula.a, f: formula.f, h: 1e-4, rng, formula: formula.prefix + &formula.formula, style: Style::vibrant()}
    }

    pub fn style(&mut self, s: Style) {
        self.style = s;
    }

    fn iterate(&self, mut state: Cplx) -> Convergence {
        let mut ctr = 0;
        let threshold = 1e-4;
        let mut tmp;

        let kernel: Box<Fn(Cplx) -> Cplx> = match self.a {
            Coef::Complex(z) => Box::new(move |state| state - z * (self.f)(state) / self.fprime(state)),
            Coef::Real(x) => Box::new(move |state| state - x * (self.f)(state) / self.fprime(state))
        };
        // this is a do while loop, mind that the "body" is actually the
        // condition and the body is empty, thus omitted
        while {
            tmp = state;
            state = kernel(state);
            ctr += 1;

            (state - tmp).norm() > threshold && ctr < 1000
        } {}
        Convergence {count: ctr, value: state}
    }

    fn fprime(&self, x: Cplx) -> Cplx {
        ((self.f)(x + self.h) - (self.f)(x - self.h)) / (2. * self.h)
    }

    fn random_formula(rng: &mut rand::StdRng) -> IterationDetails {
        // use up to 3 terms but at least 1
        let num_terms = (rng.gen_range(0f64, 1.) * 2.).floor() as i32 + 1;
        let mut terms: Vec<ComplexFunction> = Vec::new();
        let mut term_string: Vec<String> = Vec::new();

        let mut prefix;
        let a_re = (rng.gen_range(1. as Real, 2.) * 10.).floor() / 10.;
        let a_im = (rng.gen_range(1. as Real, 2.) * 10.).floor() / 10.;
        let alpha = if rng.gen::<f64>() < 0.1 {
            let tmp = Complex::new(a_re, a_im);
            prefix = format!("Generalized Newton Fractal (a = {}) of ", tmp);
            Coef::Complex(tmp)
        } else if rng.gen::<f64>() < 0.4 {
            let tmp = a_re;
            prefix = format!("Generalized Newton Fractal (a = {}) of ", tmp);
            Coef::Real(tmp)
        } else {
            prefix = "Newton Fractal of ".to_string();
            Coef::Real(1.)
        };
        prefix += "z ↦ ";

        let a_real_gen = |generator: &mut rand::StdRng| (generator.gen_range(-1. as Real, 1.) * 3. * 10.).round() / 10.;
        let a_comp_gen = |generator: &mut rand::StdRng| Complex::new(a_real_gen(generator), a_real_gen(generator));

        let mut possible_terms = Terms::new();
        // chance that all coefficients will be real
        let always_real = rng.gen_range(0f64, 1.) < 0.5;

        for _ in 0..num_terms {
            // let a be a complex number in 30% of all cases
            let a = if !always_real && rng.gen_range(0f64, 1.) < 0.3 {
                        Coef::Complex(a_comp_gen(rng))
                    } else {
                        Coef::Real(a_real_gen(rng))
                    };

            let neo = possible_terms.choice(a, rng);
            terms.push(neo.callable);
            term_string.push(neo.readable);
        }

        let f = move |x| terms.iter()
                              .map(move |f| f(x))
                              .fold(Complex {re: 0., im: 0.}, |sum, x| sum + x);

        IterationDetails {f: Box::new(f), a: alpha, formula: term_string.join(" + "), prefix: prefix}
    }

    pub fn raster(&self, x: i32, y: i32, xscale: f64, yscale: f64) -> Vec<Convergence> {
        let pixels: Vec<(i32, i32)> = iproduct!(0..y, 0..x).collect();
        pixels.par_iter()
              .map(|&(j, i)| {
                  let xp = (i-x/2) as f64 * xscale;
                  let yp = (j-y/2) as f64 * yscale;
                  let p = Complex {re: xp as Real, im: yp as Real};
                  self.iterate(p)
              })
              .collect()
    }

    pub fn render(&mut self, resolution: (i32, i32), filename: &str) -> io::Result<i64> {
        let (x, y) = resolution;

        // use randomness to determine the colors
        let random_color = self.rng.gen_range(0f64, 1.);
        let random_count = self.rng.gen_range(0f64, 1.);
        let random_zoom = self.rng.gen_range(0.1f64, 2.);
        let scale = 4e-3 * random_zoom;

        let states = self.raster(x, y, scale, scale);
        let total_iterations: i64 = states.par_iter()
                                     .map(|i| i.count)
                                     .sum();
        info!("{:.2}M iterations", total_iterations as f64/1e6);

        let style = Style::index(self.rng.gen_range(0, Style::num()));
        info!("use style '{}'", style);
        info!("rcol {}", random_color);
        info!("rcnt {}", random_count);
        info!("rzo {}", random_zoom);

        let tmp_buffer: Vec<Vec<u8>> = states.par_iter()
                            .map(|i| {
                                let hsv = (style.callable)(i, Some(random_color), Some(random_count));

                                let color::RGB(r, g, b) = hsv.to_rgb();
                                let a = 255;

                                vec![(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8, a]
                            })
                            .collect();
        let buffer: Vec<u8> = tmp_buffer.into_iter()
                                        .flatten()
                                        .collect();

        let path = Path::new(filename);
        let file = File::create(path)?;
        let w = io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, x as u32, y as u32);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        writer.write_image_data(&buffer)?; // Save

        Ok(total_iterations)
    }
}
