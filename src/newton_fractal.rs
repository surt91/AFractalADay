extern crate std;
extern crate num;
extern crate png;
extern crate rayon;

use self::num::complex::Complex;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
// To use encoder.set()
use self::png::HasParameters;

use self::rayon::prelude::*;

pub struct NewtonFractal {
    pub a: f64,
    pub f: fn(Complex<f64>) -> Complex<f64>,
    pub fprime: fn(Complex<f64>) -> Complex<f64>
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
    fn iterate(&self, mut state: Complex<f64>) -> Convergence {
        let mut ctr = 0;
        let threshold = 1e-4;
        let mut tmp;
        // *attention*: this is a do while loop, mind that the "body" is actually the
        // condition and the body is empty, thus omitted
        while {
            tmp = state;
            state = state - self.a * (self.f)(state) / (self.fprime)(state);
            ctr += 1;

            (state - tmp).norm() > threshold && ctr < 10000
        } {}
        Convergence {count: ctr, value: state}
    }

    fn raster(&self, x: i32, y: i32, xscale: f64, yscale: f64) -> Vec<Convergence> {
        let coordinates: Vec<(i32, i32)> = iproduct!(0..y, 0..x).collect();
        coordinates.par_iter()
                   .map(|&(j, i)| {
            let xp = (i-x/2) as f64 * xscale;
            let yp = (j-y/2) as f64 * yscale;
            let p = Complex {re: xp, im: yp};
            let state = self.iterate(p);
            state
        }).collect()
    }

    pub fn render(&self, filename: &str) {
        let x = 1920;
        let y = 1080;
        let states = self.raster(x, y, 2e-3, 2e-3);
        let mut buffer: Vec<u8> = Vec::with_capacity((4*x*y) as usize);
        for i in states.iter() {
            let hue = i.value.re - i.value.re.floor();
            let saturation = 1f64;
            let value = if i.count as f64 / 50. > 1f64 { 1f64 } else { i.count as f64 / 50. };

            let (r, g, b) = hsv2rgb(hue, saturation, value);
            let a = 255;

            buffer.push((r * 255.) as u8);
            buffer.push((g * 255.) as u8);
            buffer.push((b * 255.) as u8);
            buffer.push(a);
        }

        let path = Path::new(filename);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, x as u32, y as u32);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(&buffer).unwrap(); // Save
    }
}
