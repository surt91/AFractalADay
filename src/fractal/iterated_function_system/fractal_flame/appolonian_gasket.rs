use log::{debug, info};

use crate::{color::RGB, histogram::BoundsTypes};
use super::{Transformation, NonlinearTransformation, FractalFlame, AffineTransformation};
use crate::fractal::FractalBuilder;

use rand::Rng;

use crate::numbers::{Real,Cplx};
use std::f64::consts::PI as PI_;
const PI: Real = PI_ as Real;

impl FractalBuilder
{
    pub fn appolonian_gasket(self) -> FractalFlame {
        let mut rng = self.seed_rng();

        let number_of_functions = 3;
        let probabilities = vec![0.33, 0.66, 1.];

        let colors = vec![
            Some(RGB(1., 0., 0.)),
            Some(RGB(0., 1., 0.)),
            Some(RGB(0., 0., 1.)),
        ];

        let transformations = vec![
            Transformation::mobius(
                Cplx::new((3. as Real).sqrt() - 1., 0.),
                Cplx::new(1., 0.),
                Cplx::new(-1., 0.),
                Cplx::new((3. as Real).sqrt() + 1., 0.)
            ),
            Transformation::Affine(
                AffineTransformation::rotate(2.*PI/3.)
            ),
            Transformation::Affine(
                AffineTransformation::rotate(4.*PI/3.)
            )
        ];

        let description = "Appolonian Gasket".to_owned();

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
        let vibrancy = self.vibrancy.unwrap_or_else(|| rng.gen());
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
