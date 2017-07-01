mod fractal_flame;
pub mod iterated_function_system_builder;

extern crate rand;

use std::f64;
use std::io;
use itertools::Itertools;

use numbers::Real;
use color::{RGB, RGBA, HSV, color_variance};
use png;
use histogram::{bounds_without_outliers, ColoredHistogram};

use self::fractal_flame::FractalFlameSampler;

extern crate num_cpus;
use std::thread;
use std::sync::mpsc::channel;

/// The `IteratedFunctionSystem` trait applies to all ``Chaos Game type'' fractals.
pub trait IteratedFunctionSystem : Sync {
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
        let values: Vec<([Real; 2], RGB)> = sampler.skip(1000)
                                                   .take((x * y) as usize)
                                                   .collect();
        // read bounds from sample
        let b = bounds_without_outliers(values.iter().map(|&(ref z, _)| z), 1000);

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
