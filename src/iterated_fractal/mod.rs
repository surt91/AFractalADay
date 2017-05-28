mod newton_fractal;
pub mod iterated_fractal_builder;
pub mod style;

extern crate rand;
use self::rand::Rng;

extern crate rayon;
use self::rayon::prelude::*;

extern crate png;
use self::png::HasParameters;

use itertools::Itertools;

use std::path::Path;
use std::fs::File;
use std::io;

use numbers::{Coef, Real, Cplx};
use color;
use self::style::Style;

pub struct Convergence {
    pub count: i64,
    pub value: Cplx
}

/// The IteratedFractal trait applies to all ``Julia set type'' fractals, i.e., all fractals
/// that can be visualized by assigning every pixel a color dependent on a value and an iteration
/// count.
pub trait IteratedFractal : Sync {
    fn iterate(&self, state: Cplx) -> Convergence;
    fn get_rng(&mut self) -> &mut rand::StdRng;
    fn get_style(&self) -> &Style;

    fn random_coef(rng: &mut rand::StdRng) -> Coef {
        let a_re = (rng.gen_range(1. as Real, 2.) * 10.).floor() / 10.;
        let a_im = (rng.gen_range(1. as Real, 2.) * 10.).floor() / 10.;
        if rng.gen::<f64>() < 0.1 {
            let tmp = Cplx::new(a_re, a_im);
            Coef::Complex(tmp)
        } else if rng.gen::<f64>() < 0.4 {
            let tmp = a_re;
            Coef::Real(tmp)
        } else {
            Coef::Real(1.)
        }
    }

    fn raster(&self, x: i32, y: i32, xscale: f64, yscale: f64) -> Vec<Convergence> {
        let pixels: Vec<(i32, i32)> = iproduct!(0..y, 0..x).collect();
        pixels.par_iter()
              .map(|&(j, i)| {
                  let xp = (i-x/2) as f64 * xscale;
                  let yp = (j-y/2) as f64 * yscale;
                  let p = Cplx {re: xp as Real, im: yp as Real};
                  self.iterate(p)
              })
              .collect()
    }

    fn render(&mut self, resolution: (i32, i32), filename: &str) -> io::Result<f64> {
        let (x, y) = resolution;

        // use randomness to determine the colors
        let random_color = self.get_rng().gen_range(0f64, 1.);
        let random_count = self.get_rng().gen_range(0f64, 1.);
        let random_zoom = self.get_rng().gen_range(0.1f64, 2.);
        let scale = 4e-3 * random_zoom;

        let states = self.raster(x, y, scale, scale);
        let total_iterations: i64 = states.par_iter()
                                     .map(|i| i.count)
                                     .sum();
        info!("{:.2}M iterations", total_iterations as f64/1e6);

        info!("use style '{}'", self.get_style());
        info!("rcol {}", random_color);
        info!("rcnt {}", random_count);
        info!("rzo {}", random_zoom);

        let hsv: Vec<color::HSV> = states.par_iter()
                                         .map(|i| (self.get_style().callable)(i, Some(random_color), Some(random_count)))
                                         .collect();

        let var = color::color_variance(&hsv);
        info!("variance: {}", var);

        let tmp_buffer: Vec<Vec<u8>> = hsv.par_iter()
                            .map(|hsv| {
                                let color::RGB(r, g, b) = hsv.to_rgb();
                                let a = 255;

                                vec![(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8, a]
                            })
                            .collect();
        let buffer: Vec<u8> = tmp_buffer.into_iter()
                                        .flatten()
                                        .collect();

        let tmp = filename;
        let path = Path::new(&tmp);
        let file = File::create(path)?;
        let w = io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, x as u32, y as u32);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        writer.write_image_data(&buffer)?; // Save

        Ok(var)
    }
}
