use log::{debug, info};
use rand::Rng;

use crate::color::{HSV, RGB};
use super::{Transformation, NonlinearTransformation, FractalFlame};
use crate::fractal::FractalBuilder;

use crate::numbers::Real;

use std::f64::consts::PI as PI_;
const PI: Real = PI_ as Real;

impl FractalBuilder
{
    pub fn pythagorean_tree(self) -> FractalFlame {
        let mut rng = self.seed_rng();

        let number_of_functions = 3;
        let probabilities = vec![0.33, 0.66, 1.];

        let mut colors: Vec<Option<RGB>> = Vec::new();
        for _ in 0..number_of_functions {
            let hsv = HSV(rng.gen(), 1., 1.);
            colors.push(Some(hsv.to_rgb()));
        }

        let alpha: Real = rng.gen_range(0., PI/2.);
        let transformations = vec![
            Transformation::affine(alpha.cos().powi(2), -alpha.cos()*alpha.sin(), 0.,
                                   alpha.cos()*alpha.sin(), alpha.cos().powi(2), 1.),
            Transformation::affine(alpha.sin().powi(2), alpha.cos()*alpha.sin(), alpha.cos().powi(2),
                                   -alpha.cos()*alpha.sin(), alpha.sin().powi(2), 1.+alpha.cos()*alpha.sin()),
            Transformation::affine(1., 0., 0.,
                                   0., 1., 0.),
        ];

        let description = format!("Pythagorean Tree (α = {:.1}°)", alpha/PI*180.);

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
