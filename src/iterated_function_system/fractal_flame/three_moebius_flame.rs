extern crate rand;
use self::rand::Rng;

use color::{HSV, RGB};
use super::{Transformation, MobiusTransformation, NonlinearTransformation, FractalFlame};
use super::IteratedFunctionSystemBuilder;

use numbers::{Real, Cplx};

use std::f64::consts::PI as PI_;
const PI: Real = PI_ as Real;

impl IteratedFunctionSystemBuilder {
    pub fn three_moebius_flame(self) -> FractalFlame {
        let mut rng: rand::StdRng = match self.seed {
            Some(x) => { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) },
            None => rand::StdRng::new().unwrap()
        };

        let seed = match self.seed {
            Some(x) => x,
            None => 1
        };

        let number_of_functions = 3;
        let probabilities = vec![0.33, 0.66, 1.];

        let mut colors: Vec<RGB> = Vec::new();
        for _ in 0..number_of_functions {
            let hsv = HSV(rng.gen(), 1., 1.);
            colors.push(hsv.to_rgb());
        }

        let f1 = Cplx::from_polar(&1., &(2.*PI/3.));
        let t1 = MobiusTransformation::random(&mut rng);
        let transformations = vec![
            Transformation::Mobius(t1.clone()),
            Transformation::Mobius(t1.clone()*f1),
            Transformation::Mobius(t1.clone()*f1*f1),
        ];

        let variation = match self.variation {
            Some(v) => NonlinearTransformation::new(v),
            None => NonlinearTransformation::random(&mut rng)
        };

        let description = format!("Möbius Flame (3): '{}' Variation",
                                   variation.name());

        info!("Will render {}", description);

        debug!("number of functions    : {:?}", number_of_functions);
        debug!("cumulative probabilites: {:?}", probabilities);
        debug!("colors                 : {:?}", colors);
        debug!("affine transformations : {:?}", transformations);
        debug!("Variation              : {:?}", variation);

        FractalFlame {
            rng,
            description,
            seed,
            number_of_functions,
            probabilities,
            colors,
            transformations,
            variation,
            strict_bounds: false
        }
    }
}
