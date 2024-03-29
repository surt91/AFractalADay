use log::{debug, info};

use crate::{color::RGB, histogram::BoundsTypes};
use super::{Transformation, NonlinearTransformation, FractalFlame};
use crate::fractal::FractalBuilder;

impl FractalBuilder
{
    pub fn barnsley_fern(self) -> FractalFlame {
        let rng = self.seed_rng();

        let number_of_functions = 4;
        let probabilities = vec![0.01, 0.86, 0.93, 1.];

        let colors = vec![
            Some(RGB(0.5, 0.5, 0.1)),
            Some(RGB(0.02, 0.6, 0.001)),
            Some(RGB(0.8, 0.8, 0.)),
            Some(RGB(0.8, 0.6, 0.))
        ];
        let transformations = vec![
            Transformation::affine(0., 0., 0., 0., 0.16, 0.),
            Transformation::affine(0.85, 0.04, 0., -0.04, 0.85, 1.6),
            Transformation::affine(0.2, -0.26, 0., 0.23, 0.22, 1.6),
            Transformation::affine(-0.15, 0.28, 0., 0.26, 0.24, 0.44),
        ];

        let description = "Barnsley Fern".to_owned();

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

        let gamma = self.gamma.unwrap_or(4.);
        let vibrancy = self.vibrancy.unwrap_or(0.4);
        let bounds = self.bounds.unwrap_or(BoundsTypes::StrictBounds);

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
            bounds,
            gamma,
            vibrancy,
        }
    }
}
