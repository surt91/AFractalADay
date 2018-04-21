mod newton;
mod julia;
mod mandelbrot;
pub mod style;

extern crate rayon;
use self::rayon::prelude::*;

use numbers::{Real, Cplx};
use color;
use self::style::Stylable;
use super::estimate_quality_after;

use super::RngType;

pub struct Convergence {
    pub count: f64,
    pub value: Cplx
}

/// The `EscapeTimeFractal` trait applies to all ``Julia set type'' fractals, i.e., all fractals
/// that can be visualized by assigning every pixel a color dependent on a value and an iteration
/// count.
pub trait EscapeTimeFractal : Sync + Stylable {
    fn description(&self) -> &str;
    fn iterate(&self, state: Cplx) -> Convergence;
    fn get_rng(&mut self) -> &mut RngType;

    fn raster(&self, resolution: (u32, u32), scale: (f64, f64), center: (f64, f64)) -> Vec<Convergence> {
        let (x, y) = resolution;
        let (x, y) = (x as i32, y as i32);
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

    // TODO: implement supersampling
    fn render(&mut self, resolution: (u32, u32),
                         scale: Option<f64>,
                         center: Option<(f64, f64)>)
        -> (Vec<u8>, bool)
    {
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

        let buffer: Vec<u8> = hsv.par_iter()
                                .map(|hsv| {
                                    let color::RGB(r, g, b) = hsv.to_rgb();
                                    let a = 255;

                                    vec![(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8, a]
                                })
                                .flatten()
                                .collect();

        let rgb: Vec<color::RGBA> = hsv.iter().map(|c| c.to_rgba()).collect();
        let good = estimate_quality_after(&rgb, &resolution);

        (buffer, good)
    }
}
