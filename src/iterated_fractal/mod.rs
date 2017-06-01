mod newton;
mod julia;
mod mandelbrot;
pub mod iterated_fractal_builder;
pub mod style;

extern crate rand;

extern crate rayon;
use self::rayon::prelude::*;

extern crate png;
use self::png::HasParameters;

use itertools::Itertools;

use std::path::Path;
use std::fs::File;
use std::io;

use numbers::{Real, Cplx};
use color;
use self::style::Stylable;

pub struct Convergence {
    pub count: f64,
    pub value: Cplx
}

/// The `IteratedFractal` trait applies to all ``Julia set type'' fractals, i.e., all fractals
/// that can be visualized by assigning every pixel a color dependent on a value and an iteration
/// count.
pub trait IteratedFractal : Sync + Stylable {
    fn description(&self) -> &str;
    fn iterate(&self, state: Cplx) -> Convergence;
    fn get_rng(&mut self) -> &mut rand::StdRng;

    fn raster(&self, resolution: (i32, i32), scale: (f64, f64), center: (f64, f64)) -> Vec<Convergence> {
        let (x, y) = resolution;
        let (xscale, yscale) = scale;
        let (cx, cy) = center;
        let pixels: Vec<(i32, i32)> = iproduct!(0..y, 0..x).collect();
        pixels.par_iter()
              .map(|&(j, i)| {
                  let xp = (i-x/2) as f64 * xscale + cx;
                  let yp = (j-y/2) as f64 * yscale + cy;
                  let p = Cplx {re: xp as Real, im: yp as Real};
                  self.iterate(p)
              })
              .collect()
    }

    fn render(&mut self, resolution: (i32, i32),
                         scale: Option<f64>,
                         center: Option<(f64, f64)>,
                         filename: &str) -> io::Result<f64> {
        let scale = match scale {
            Some(x) => (x, x),
            None => {
                let x = 1. / resolution.1 as f64;
                (x, x)
            }
        };

        let center = match center {
            Some(x) => x,
            None => (0., 0.)
        };

        let states = self.raster(resolution, scale, center);
        let total_iterations: i64 = states.par_iter()
                                     .map(|i| i.count as i64)
                                     .sum();
        info!("{:.2}M iterations", total_iterations as f64/1e6);

        let hsv: Vec<color::HSV> = states.par_iter()
                                         .map(|i| self.style(i))
                                         .collect();

        let var = color::color_variance(&hsv);
        info!("variance: {}", var);

        let buffer: Vec<u8> = hsv.iter()
                                .map(|hsv| {
                                    let color::RGB(r, g, b) = hsv.to_rgb();
                                    let a = 255;

                                    vec![(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8, a]
                                })
                                .flatten()
                                .collect();

        let tmp = filename;
        let path = Path::new(&tmp);
        let file = File::create(path)?;
        let w = io::BufWriter::new(file);

        let (x, y) = resolution;
        let mut encoder = png::Encoder::new(w, x as u32, y as u32);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        writer.write_image_data(&buffer)?; // Save

        Ok(var)
    }
}
