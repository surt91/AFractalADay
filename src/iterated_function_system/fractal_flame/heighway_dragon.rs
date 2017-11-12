extern crate rand;
use self::rand::Rng;

use color::{HSV, RGB};
use super::{Transformation, NonlinearTransformation, Variation, FractalFlame};
use super::IteratedFunctionSystemBuilder;
use super::RngType;

use numbers::Real;

use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_4};
const PI_QUARTER: Real = FRAC_PI_4 as Real;
const BY_SQRT: Real = FRAC_1_SQRT_2 as Real;

impl IteratedFunctionSystemBuilder
{
    pub fn heighway_dragon(self) -> FractalFlame<RngType> {
        let mut rng = self.seed_rng();

        let number_of_functions = 2;
        let probabilities = vec![0.5, 1.];

        let mut colors: Vec<RGB> = Vec::new();
        for _ in 0..number_of_functions {
            let hsv = HSV(rng.gen(), 1., 1.);
            colors.push(hsv.to_rgb());
        }

        let transformations = vec![
            Transformation::affine(PI_QUARTER.cos() * BY_SQRT,
                                   (-PI_QUARTER).sin() * BY_SQRT,
                                   0.,
                                   PI_QUARTER.sin() * BY_SQRT,
                                   PI_QUARTER.cos() * BY_SQRT,
                                   0.),
            Transformation::affine((3.*PI_QUARTER).cos() * BY_SQRT,
                                   (-3.*PI_QUARTER).sin() * BY_SQRT,
                                   BY_SQRT,
                                   (3.*PI_QUARTER).sin() * BY_SQRT,
                                   (3.*PI_QUARTER).cos() * BY_SQRT,
                                   (2. as Real).sqrt() * BY_SQRT) ,
        ];

        let mut description = "Heighway Dragon".to_owned();

        let variation = match self.variation {
            Some(v) => {
                description.push_str(&format!(" with Variation '{}'", v.name()));
                NonlinearTransformation::new(v)
            },
            None => NonlinearTransformation::new(Variation::Linear)
        };

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
            strict_bounds: true
        }
    }
}
