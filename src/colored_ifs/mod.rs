mod fractal_flame;
pub mod colored_ifs_builder;

extern crate rand;

use std::f64;
use std::io;
use itertools::Itertools;

use numbers::Real;
use color::{RGB, RGBA};
use png;
use histogram::{bounds, ColoredHistogram};

use self::fractal_flame::FractalFlameSampler;

/// The `ColoredIFS` trait applies to all ``Chaos Game type'' fractals.
pub trait ColoredIFS : Sync {
    fn description(&self) -> &str;
    fn get_rng(&mut self) -> &mut rand::StdRng;
    fn get_sampler(&mut self) -> FractalFlameSampler;

    // TODO: implement supersampling
    fn render(&mut self, resolution: (u32, u32),
                         samples_per_pixel: usize,
                         filename: &str) -> io::Result<f64> {
        let (x, y) = resolution;

        let sampler = self.get_sampler();

        // warm up and get sample to derive bounds
        let values: Vec<([Real; 2], RGB)> = sampler.skip(100).take((x * y) as usize).collect();
        // read bounds from sample
        let b = bounds(values.iter().map(|&(ref z, _)| z));
        let mut hist = ColoredHistogram::new(resolution, b);

        hist.feed(values.into_iter());
        
        let sampler = self.get_sampler();
        hist.feed(sampler.take((samples_per_pixel - 1) * (x * y) as usize));
        // generate histogram, using the sample and new values
        let hist = histogram_colored(values.into_iter()
                                           .chain(
                                               self.take((samples_per_pixel-1) * (x * y) as usize)
                                           ),
                                     resolution, b);

        let buffer: Vec<u8> = hist.iter()
                                  .map(|rgba| {
                                      let &RGBA(r, g, b, a) = rgba;

                                      vec![r, g, b, a]
                                  })
                                  .flatten()
                                  .collect();

        png::save_png(filename, x, y, &buffer)?;
        Ok(1.)
    }
}
