use rand::Rng;

use crate::color::{HSV, RGB};
use super::{Transformation, NonlinearTransformation, FractalFlame};
use crate::fractal::FractalBuilder;

use crate::numbers::Real;

use std::f64::consts::PI as PI_;
const PI: Real = PI_ as Real;
// const R: Real = (3. - (5. as Real).sqrt())/2.;
const R: Real = (3. - 2.23606797749979)/2.;

impl FractalBuilder
{
    pub fn sierpinski_pentagon(self) -> FractalFlame {
        let mut rng = self.seed_rng();

        let number_of_functions = 5;
        let probabilities = vec![0.2, 0.4, 0.6, 0.8, 1.];

        let mut colors: Vec<Option<RGB>> = Vec::new();
        for _ in 0..number_of_functions {
            let hsv = HSV(rng.gen(), 1., 1.);
            colors.push(Some(hsv.to_rgb()));
        }

        let x3 = R * (1. + (72.*PI/180.).cos() + (36.*PI/180.).cos());
        let y3 = R * ((72.*PI/180.).sin() + (36.*PI/180.).sin());
        let x4 = R * (36.*PI/180.).cos();
        let y4 = R * (2.*(72.*PI/180.).sin() + (36.*PI/180.).sin());
        let x5 = R * (-(72.*PI/180.).cos() + (36.*PI/180.).cos() - 1.);
        let transformations = vec![
            Transformation::affine(R, 0., 0.,
                                   0., R, 0.),
            Transformation::affine(R, 0., 1.-R,
                                   0., R, 0.),
            Transformation::affine(R, 0., x3,
                                   0., R, y3),
            Transformation::affine(R, 0., x4,
                                   0., R, y4),
            Transformation::affine(R, 0., x5,
                                   0., R, y3),
        ];

        let description = "Sierpinski Pentagon".to_owned();

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
            None => rng.gen()
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
