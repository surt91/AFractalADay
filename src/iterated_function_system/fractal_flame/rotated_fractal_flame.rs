extern crate rand;
use self::rand::Rng;

use itertools;

use color::{HSV, RGB};
use super::{Transformation, AffineTransformation, NonlinearTransformation, FractalFlame};
use super::IteratedFunctionSystemBuilder;
use super::RngType;

use numbers::Real;

use std::f64::consts::PI as PI_;
const PI: Real = PI_ as Real;

impl IteratedFunctionSystemBuilder
{
    pub fn rotated_fractal_flame(self) -> FractalFlame<RngType> {
        let mut rng = self.seed_rng();

        let number_of_functions = rng.gen_range(2, 7);
        let number_of_symmetries = rng.gen_range(2, 7);

        let prob: Vec<f64> = rng.gen_iter().take(number_of_functions).collect();
        let mut p = 0.;
        let p_norm: f64 = prob.iter().sum();
        let mut probabilities: Vec<f64> = Vec::new();
        for i in prob {
            p += i/p_norm/number_of_symmetries as f64;
            probabilities.push(p);
        }

        let mut colors: Vec<RGB> = Vec::new();
        for _ in 0..number_of_functions {
            let hsv = HSV(rng.gen(), 1., 1.);
            colors.push(hsv.to_rgb());
        }
        let mut transformations: Vec<Transformation> =
                itertools::repeat_call(|| Transformation::Affine(AffineTransformation::random(&mut rng)))
                          .take(number_of_functions)
                          .collect();

        let variation = match self.variation {
            Some(v) => NonlinearTransformation::new(v),
            None => NonlinearTransformation::random(&mut rng)
        };

        // plus one for the symmetry
        for i in 1..(number_of_symmetries) {
            p += 1./number_of_symmetries as f64;
            probabilities.push(p);
            // black will be treated as transparent
            // FIXME: .make colors Vec<Option<RGB>>
            let hsv = HSV(0., 0., 0.);
            colors.push(hsv.to_rgb());
            transformations.push(Transformation::Affine(AffineTransformation::rotate(2.*PI/number_of_symmetries as Real * i as Real)));
        }

        let description = format!("Symmetric Fractal Flame: '{}' Variation, {} affine transformations, {} symmetries",
                                   variation.name(),
                                   number_of_functions,
                                   number_of_symmetries);

        let number_of_functions = number_of_functions + number_of_symmetries;

        info!("Will render {}", description);

        debug!("number of functions    : {:?}", number_of_functions);
        debug!("cumulative probabilites: {:?}", probabilities);
        debug!("colors                 : {:?}", colors);
        debug!("affine transformations : {:?}", transformations);
        debug!("Variation              : {:?}", variation);

        FractalFlame {
            rng,
            description,
            number_of_functions,
            probabilities,
            colors,
            transformations,
            variation,
            strict_bounds: false
        }
    }
}
