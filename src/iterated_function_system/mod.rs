mod heighway_dragon;
mod barnsly_fern;
pub mod iterated_function_system_builder;

extern crate rand;

use std::f64;
use std::io;
use itertools::Itertools;

use numbers::Real;
use color;
use png;
use histogram::{bounds, histogram};


/// The `IteratedFunctionSystem` trait applies to all ``Chaos Game type'' fractals.
pub trait IteratedFunctionSystem : Sync + Iterator<Item=[Real; 2]> {
    fn description(&self) -> &str;
    fn get_rng(&mut self) -> &mut rand::StdRng;

    // TODO: implement supersampling
    fn render(&mut self, resolution: (u32, u32),
                         samples_per_pixel: usize,
                         filename: &str) -> io::Result<f64> {
        let (x, y) = resolution;

        // warm up and get sample to derive bounds
        let values: Vec<[Real; 2]> = self.skip(100).take((x * y) as usize).collect();
        // read bounds from sample
        let b = bounds(values.iter());
        // generate histogram, using the sample and new values
        let hist = histogram(values.into_iter()
                                   .chain(
                                       self.take((samples_per_pixel-1) * (x * y) as usize)
                                   ),
                             resolution, b);

        let max_val = hist.iter().max().unwrap();
        // TODO: maybe color by distance of the jump?
        // TODO: implement fractal flame (http://flam3.com/flame_draves.pdf)
        // TODO: use color (multiple histograms for color channels + alpha (total count))
        let hsv: Vec<color::HSV> = hist.iter()
                                       .map(|i| color::HSV(0., 0., (*i as f64).ln() / (*max_val as f64).ln()))
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

        png::save_png(filename, x, y, &buffer)?;
        Ok(var)
    }
}
