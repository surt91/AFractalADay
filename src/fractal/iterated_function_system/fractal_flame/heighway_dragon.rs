extern crate rand;
use self::rand::Rng;

use color::{HSV, RGB};
use super::{Transformation, NonlinearTransformation, FractalFlame};
use fractal::FractalBuilder;

use numbers::Real;

use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_4};
const PI_QUARTER: Real = FRAC_PI_4 as Real;
const BY_SQRT: Real = FRAC_1_SQRT_2 as Real;

impl FractalBuilder
{
    pub fn heighway_dragon(self) -> FractalFlame {
        let mut rng = self.seed_rng();

        let number_of_functions = 2;
        let probabilities = vec![0.5, 1.];

        let mut colors: Vec<Option<RGB>> = Vec::new();
        for _ in 0..number_of_functions {
            let hsv = HSV(rng.gen(), 1., 1.);
            colors.push(Some(hsv.to_rgb()));
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

        let description = "Heighway Dragon".to_owned();

        let variation = match self.variation {
            Some(v) => NonlinearTransformation::new(v),
            None => NonlinearTransformation::identity()
        };

        let post_transform = match self.post_transform {
            Some(v) => v,
            None => Transformation::identity()
        };

        let final_transform = match self.final_transform {
            Some(v) => NonlinearTransformation::new(v),
            None => NonlinearTransformation::identity()
        };

        let final_color = None;

        let gamma = match self.gamma {
            Some(s) => s,
            None => 4.
        };

        let vibrancy = match self.vibrancy {
            Some(s) => s,
            None => 0.4
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
            post_transform,
            final_transform,
            final_color,
            strict_bounds: true,
            gamma,
            vibrancy,
        }
    }
}
