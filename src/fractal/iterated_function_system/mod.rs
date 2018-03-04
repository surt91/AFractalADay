mod fractal_flame;
mod quality;
pub mod variation;
pub mod symmetry;

mod transformation;
pub use self::transformation::{Transformation,AffineTransformation,MobiusTransformation,NonlinearTransformation};

pub mod serialize;

use rand::Rng;

use std::f64;
use itertools::Itertools;

use numbers::Real;
use color::{RGB, RGBA};
use histogram::{bounds_without_outliers, bounds_zoom, ColoredHistogram};
use self::quality::probably_good;
use self::serialize::IteratedFunctionSystemConfig;

use super::estimate_quality_after;
use super::quality::downscale;

extern crate num_cpus;
use std::thread;
use std::sync::mpsc::channel;

use super::{RngType, SeedType};

/// The `IteratedFunctionSystem` trait applies to all ``Chaos Game type'' fractals.
pub trait IteratedFunctionSystem : Sync {
    fn description(&self) -> &str;
    fn needs_strict_bounds(&self) -> bool;
    fn gamma(&self) -> f64;
    fn vibrancy(&self) -> f64;
    fn get_rng(&mut self) -> &mut RngType;
    fn get_sampler(&mut self) -> IteratedFunctionSystemSampler<RngType>;
    fn get_serializable(&self) -> IteratedFunctionSystemConfig;

    fn estimate_quality_before(&mut self) -> bool {
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

    fn render(&mut self, resolution: (u32, u32),
                         samples_per_pixel: usize,
                         supersampling: bool
        )
        -> (Vec<u8>, bool)
    {
        let (x, y) = resolution;

        let (x, y) = if supersampling {
            (x*2, y*2)
        } else {
            (x, y)
        };

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

        let mut hist = ColoredHistogram::new((x, y), b, self.vibrancy(), self.gamma());

        let (tx, rx) = channel();
        for _ in 0..cpus {
            let tx = tx.clone();
            let sampler = self.get_sampler();
            let mut hist = hist.clone();
            thread::spawn(move || {
                hist.feed(sampler.take((iterations_per_task) * (x * y) as usize));
                tx.send(hist).unwrap();
            });
        }

        // and do the remainder in the main thread
        let remainder = (samples_per_pixel - 1) - iterations_per_task*cpus;
        let sampler = self.get_sampler();


        // feed the remainder into the main histogram
        hist.feed(sampler.take(remainder * (x * y) as usize));
        // feed the values from the bounds estimation into the histogram
        hist.feed(values.into_iter());

        for _ in 0..cpus {
            let h = rx.recv().unwrap();
            hist.merge(&h);
        }

        let rgb = hist.normalize();

        let rgb = if supersampling {
            downscale(&rgb, &(x, y))
        } else {
            rgb
        };

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

        let good = estimate_quality_after(&rgb, &resolution);
        (buffer, good)
    }
}

pub struct IteratedFunctionSystemSampler<T>
    where T: Rng
{
    rng: T,
    number_of_functions: usize,
    probabilities: Vec<f64>,
    colors: Vec<RGB>,
    transformations: Vec<Transformation>,
    variation: NonlinearTransformation,
    p: [Real; 2],
    r: f64,
    g: f64,
    b: f64,
}

impl <T> Iterator for IteratedFunctionSystemSampler<T>
    where T: Rng
{
    type Item = ([Real; 2], RGB);

    fn next(&mut self) -> Option<([Real; 2], RGB)> {
        let r = self.rng.gen::<f64>();

        let mut index = 0;
        for i in 0..self.number_of_functions {
            if r < self.probabilities[i] {
                index = i;
                break;
            }
        }

        let mut is_symmetry_transformation = false;
        let transformed = match self.transformations[index] {
            Transformation::Affine(ref x) => {
                is_symmetry_transformation = x.symmetry;
                x.transform(self.p)
            },
            Transformation::Mobius(ref x) => {
                x.transform(self.p[0], self.p[1])
            }
        };

        // do not apply variation to symmetry transforms and do not bother about linear (identity)
        if !is_symmetry_transformation {
            self.p = self.variation.transform(transformed);
        } else {
            self.p = transformed;
        }

        let RGB(r, g, b) = self.colors[index];
        // if it is black, ignore it
        // FIXME: better would be Option<RGB>
        if r != 0. || g != 0. || b != 0.
        {
            self.r = (r + self.r)/2.;
            self.g = (g + self.g)/2.;
            self.b = (b + self.b)/2.;
        }

        Some((self.p, RGB(self.r, self.g, self.b)))
    }
}
