pub mod fractal_flame;
mod quality;
pub mod variation;
pub mod symmetry;
pub mod quadratic_map;
pub mod ode;

pub mod transformation;
pub use self::transformation::{Transformation,AffineTransformation,MobiusTransformation,NonlinearTransformation};

use rand::Rng;

use std::f64;

use rayon::prelude::*;

use crate::numbers::Real;
use crate::color::{RGB, RGBA};
use crate::histogram::{BoundsTypes, bounds_without_outliers, bounds_zoom, ColoredHistogram};
use self::{ode::OdeFractal, quality::probably_good};

use super::estimate_quality_after;

use num_cpus;
use std::thread;
use std::sync::mpsc::channel;

use serde::{self, Serialize, Deserialize};

use fractal_flame::FractalFlame;
use quadratic_map::QuadraticMap;

use super::{RngType, default_rng};

#[derive(Debug, Serialize, Deserialize)]
pub enum IterationFractalType {
    IFS(FractalFlame),
    QuadraticMap(QuadraticMap),
    OdeFractal(OdeFractal),
    None,
}
pub enum SuggestedIterations {
    Absolute(usize),
    PerPixel(usize),
}

pub enum SuggestedParallelism {
    Limited(usize),
    Unlimited,
}

/// The `IteratedFunctionSystem` trait applies to all ``Chaos Game type'' fractals.
pub trait IteratedFunctionSystem : Sync {
    fn description(&self) -> &str;
    fn needs_strict_bounds(&self) -> BoundsTypes;
    fn gamma(&self) -> f64;
    fn vibrancy(&self) -> f64;
    fn get_rng(&mut self) -> &mut RngType;
    fn get_sampler(&mut self) -> Box<dyn Samplable + Send>;
    fn get_serializable(&self) -> IterationFractalType;

    fn suggested_iterations(&self) -> SuggestedIterations {
        SuggestedIterations::PerPixel(1000)
    }

    fn suggested_iterations_draft(&self) -> SuggestedIterations {
        SuggestedIterations::PerPixel(100)
    }

    fn suggested_parallelism(&self) -> SuggestedParallelism {
        SuggestedParallelism::Unlimited
    }

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
                         samples: SuggestedIterations,
                         parallelism: SuggestedParallelism,
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

        // use N-1 additional threads (where N is the number of logical CPU)
        // this way one thread is idle and can calculate the remainder and merge the results


        let (total_samples, warmup) = match samples {
            SuggestedIterations::Absolute(samples) => (
                samples,
                (0.1 * samples as f64) as usize,
            ),
            SuggestedIterations::PerPixel(samples) => (
                samples * (x * y) as usize,
                (0.01 * samples as f64 * (x * y) as f64) as usize,
            ),
        };

        let parallelism = match parallelism {
            SuggestedParallelism::Unlimited => num_cpus::get(),
            SuggestedParallelism::Limited(n) => n,
        };

        let sampler = self.get_sampler();

        // warm up and get sample to derive bounds
        // these samples will be discarded
        let values: Vec<([Real; 2], RGB)> = sampler.skip(warmup)
                                                   .take(warmup)
                                                   .collect();

        // read bounds from sample
        let b = match self.needs_strict_bounds() {
            BoundsTypes::StrictBounds => bounds_without_outliers(values.iter().map(|&(ref z, _)| z), 0),
            BoundsTypes::BoundsWithoutOutliers => bounds_without_outliers(values.iter().map(|&(ref z, _)| z), 1000),
            BoundsTypes::ZoomedBounds => bounds_zoom(values.iter().map(|&(ref z, _)| z), x as Real/y as Real),
        };

        // we might miss up to parallelism - 1 samples, but we do not care about this
        let iterations_per_task = total_samples / parallelism;

        let mut hist = ColoredHistogram::new((x, y), b, self.vibrancy(), self.gamma());

        let (tx, rx) = channel();
        for _ in 0..parallelism {
            let tx = tx.clone();
            let mut sampler = self.get_sampler();
            sampler.perturb();
            let mut hist = hist.clone();
            thread::spawn(move || {
                hist.feed(sampler.take(iterations_per_task));
                tx.send(hist).unwrap();
            });
        }

        for _ in 0..parallelism {
            let h = rx.recv().unwrap();
            hist.merge(&h);
        }

        let hist = if supersampling {

            hist.downscale()
        } else {
            hist
        };

        let rgb = hist.normalize();

        let buffer: Vec<u8> = rgb.par_iter()
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
    colors: Vec<Option<RGB>>,
    transformations: Vec<Transformation>,
    variation: NonlinearTransformation,
    post_transform: Transformation,
    final_transform: NonlinearTransformation,
    final_color: Option<RGB>,
    p: [Real; 2],
    rgb: RGB
}

impl<T> Iterator for IteratedFunctionSystemSampler<T>
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
                x.transform(self.p)
            }
        };

        // do not apply variation to symmetry transforms and do not bother about linear (identity)
        if !is_symmetry_transformation {
            self.p = self.variation.transform(transformed);
        } else {
            self.p = transformed;
        }

        self.p = self.post_transform.transform(self.p);

        if let Some(RGB(tr, tg, tb)) = self.colors[index] {
            let RGB(r, g, b) = self.rgb;
            self.rgb = RGB(
                (r + tr)/2.,
                (g + tg)/2.,
                (b + tb)/2.
            )
        };

        let p = self.final_transform.transform(self.p);
        let rgb = match self.final_color {
            Some(ref c) => {
                let RGB(rf, gf, bf) = c.clone();
                let RGB(r, g, b) = self.rgb;
                RGB(
                    (rf + r)/2.,
                    (gf + g)/2.,
                    (bf + b)/2.
                )
            },
            None => self.rgb.clone()
        };

        Some((p, rgb))
    }
}


pub trait Perturbable {
    fn perturb(&mut self);
}

pub trait Samplable : Iterator<Item=([Real; 2], RGB)> + Perturbable {}

impl<T> Perturbable for IteratedFunctionSystemSampler<T>
    where T: Rng
{
    fn perturb(&mut self) {
        self.p[0] += self.rng.gen_range(-0.01, 0.01)
    }
}

impl<T> Samplable for IteratedFunctionSystemSampler<T>
    where T: Rng {}