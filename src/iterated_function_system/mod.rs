mod fractal_flame;
mod quality;
pub mod iterated_function_system_builder;
pub mod variation;
pub mod symmetry;

extern crate rand;

use std::f64;
use std::io;
use itertools::Itertools;

use numbers::Real;
use color::{RGB, RGBA, HSV, color_variance};
use png;
use histogram::{bounds_without_outliers, bounds_zoom, ColoredHistogram};
use self::quality::probably_good;

use self::fractal_flame::FractalFlameSampler;

extern crate num_cpus;
use std::thread;
use std::sync::mpsc::channel;

pub type RngType = rand::Isaac64Rng;
pub type SeedType = [u64; 4];


/// The `IteratedFunctionSystem` trait applies to all ``Chaos Game type'' fractals.
pub trait IteratedFunctionSystem : Sync {
    fn description(&self) -> &str;
    fn needs_strict_bounds(&self) -> bool;
    fn get_rng(&mut self) -> &mut RngType;
    fn get_sampler(&mut self) -> FractalFlameSampler<RngType>;

    fn estimate_quality(&mut self) -> bool {
        let sampler = self.get_sampler();

        // warm up and get sample to derive bounds
        let values: Vec<([Real; 2], RGB)> = sampler.skip(1000)
                                                   .take(100000 as usize)
                                                   .collect();
        let coords: Vec<[Real; 2]> = values.iter()
                                           .map(|&(z, _)| z)
                                           .collect();

        // read bounds from sample
        let b = bounds_without_outliers(coords.iter(), 100);

        // estimate if the fractal will be interesting
        probably_good(&coords, b)
    }

    // TODO: implement supersampling
    fn render(&mut self, resolution: (u32, u32),
                         samples_per_pixel: usize,
                         filename: &str) -> io::Result<f64> {
        let (x, y) = resolution;

        let sampler = self.get_sampler();

        // warm up and get sample to derive bounds
        let values: Vec<([Real; 2], RGB)> = sampler.skip(1000)
                                                   .take((x * y) as usize)
                                                   .collect();

        // read bounds from sample
        let b = if self.needs_strict_bounds() {
            bounds_without_outliers(values.iter().map(|&(ref z, _)| z), 1000)
        } else {
            bounds_zoom(values.iter().map(|&(ref z, _)| z), x as f32/y as f32)
        };

        // use N-1 additional threads (where N is the number of logical CPU)
        // this way one thread is idle and can calculate the remainder and merge the results
        let cpus = num_cpus::get();
        let iterations_per_task = (samples_per_pixel - 1) / cpus;

        let (tx, rx) = channel();
        for _ in 0..cpus {
            let tx = tx.clone();
            let sampler = self.get_sampler();
            let mut hist = ColoredHistogram::new(resolution, b);
            thread::spawn(move || {
                hist.feed(sampler.take((iterations_per_task) * (x * y) as usize));
                tx.send(hist).unwrap();
            });
        }

        // and do the remainder in the main thread
        let remainder = (samples_per_pixel - 1) - iterations_per_task*cpus;
        let sampler = self.get_sampler();

        let mut hist = ColoredHistogram::new(resolution, b);
        // feed the remainder into the main histogram
        hist.feed(sampler.take(remainder * (x * y) as usize));
        // feed the values from the bounds estimation into the histogram
        hist.feed(values.into_iter());

        for _ in 0..cpus {
            let h = rx.recv().unwrap();
            hist.merge(&h);
        }

        let rgb = hist.normalize();
        let buffer: Vec<u8> = rgb.iter()
                                 .map(|rgba| {
                                     let &RGBA(r, g, b, a) = rgba;
                                     let alpha = a as f64 / 255.;
                                     // black background
                                     vec![  (r as f64 * alpha) as u8,
                                            (g as f64 * alpha) as u8,
                                            (b as f64 * alpha) as u8,
                                            255]
                                     }
                                 )
                                 .flatten()
                                 .collect();

        png::save_png(filename, x, y, &buffer)?;

        let hsv: Vec<HSV> = rgb.iter().map(|c| c.blend_black().to_hsv()).collect();
        let var = color_variance(&hsv);
        Ok(var)
    }
}
