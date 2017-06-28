extern crate rand;
use self::rand::Rng;

use color::{HSV, RGB};
use super::{AffineTransformation, NonlinearTransformation, Variation, FractalFlame};
use super::IteratedFunctionSystemBuilder;

use numbers::Real;

use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_4};
const PI_QUARTER: Real = FRAC_PI_4 as Real;
const BY_SQRT: Real = FRAC_1_SQRT_2 as Real;

impl IteratedFunctionSystemBuilder {
    pub fn heighway_dragon(self) -> FractalFlame {
        let mut rng: rand::StdRng = match self.seed {
            Some(x) => { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) },
            None => rand::StdRng::new().unwrap()
        };

        let seed = match self.seed {
            Some(x) => x,
            None => 1
        };

        let number_of_functions = 2;
        let probabilities = vec![0.5, 1.];

        let mut colors: Vec<RGB> = Vec::new();
        for _ in 0..number_of_functions {
            let hsv = HSV(rng.gen(), 1., 1.);
            colors.push(hsv.to_rgb());
        }

        let affine_transformations = vec![
            AffineTransformation::new(PI_QUARTER.cos(),
                                      (-PI_QUARTER).sin(),
                                      0.,
                                      PI_QUARTER.sin(),
                                      PI_QUARTER.cos(),
                                      0.) * BY_SQRT,
            AffineTransformation::new((3.*PI_QUARTER).cos(),
                                      (-3.*PI_QUARTER).sin(),
                                      1.,
                                      (3.*PI_QUARTER).sin(),
                                      (3.*PI_QUARTER).cos(),
                                      (2. as Real).sqrt()) * BY_SQRT,
        ];
        let nonlinear_transformation = NonlinearTransformation::new(Variation::Linear);

        let description = "Heighway Dragon".to_owned();

        info!("Will render {}", description);

        debug!("number of functions    : {:?}", number_of_functions);
        debug!("cumulative probabilites: {:?}", probabilities);
        debug!("colors                 : {:?}", colors);
        debug!("affine transformations : {:?}", affine_transformations);
        debug!("Variation              : {:?}", nonlinear_transformation);

        FractalFlame {
            rng,
            description,
            seed,
            number_of_functions,
            probabilities,
            colors,
            affine_transformations,
            nonlinear_transformation
        }
    }
}
