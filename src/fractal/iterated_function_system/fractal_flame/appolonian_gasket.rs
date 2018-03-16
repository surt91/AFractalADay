use color::RGB;
use super::{Transformation, NonlinearTransformation, FractalFlame, AffineTransformation};
use fractal::{FractalBuilder, RngType};

use numbers::{Real,Cplx};
use std::f64::consts::PI as PI_;
const PI: Real = PI_ as Real;

impl FractalBuilder
{
    pub fn appolonian_gasket(self) -> FractalFlame<RngType> {
        let rng = self.seed_rng();

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

        let gamma = match self.gamma {
            Some(s) => s,
            None => 4.
        };

        let vibrancy = match self.vibrancy {
            Some(s) => s,
            None => 0.8
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
