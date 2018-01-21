extern crate rand;
use self::rand::Rng;

use color::{HSV, RGB};
use super::{Transformation, NonlinearTransformation, Variation, FractalFlame};
use fractal::FractalBuilder;
use super::RngType;

use numbers::Real;

use std::f64::consts::PI as PI_;
const PI: Real = PI_ as Real;

impl FractalBuilder
{
    pub fn pythagorean_tree(self) -> FractalFlame<RngType> {
        let mut rng = self.seed_rng();

        let number_of_functions = 3;
        let probabilities = vec![0.33, 0.66, 1.];

        let mut colors: Vec<RGB> = Vec::new();
        for _ in 0..number_of_functions {
            let hsv = HSV(rng.gen(), 1., 1.);
            colors.push(hsv.to_rgb());
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

        let mut description = format!("Pythagorean Tree (α = {:.1}°)", alpha/PI*180.);

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
