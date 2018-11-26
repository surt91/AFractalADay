use rand::Rng;

use color::{HSV, RGB};
use super::{Transformation, NonlinearTransformation, FractalFlame};
use fractal::FractalBuilder;

use numbers::Real;

impl FractalBuilder
{
    pub fn sierpinski_gasket(self) -> FractalFlame {
        let mut rng = self.seed_rng();

        let number_of_functions = 3;
        let probabilities = vec![0.33, 0.66, 1.];

        let mut colors: Vec<Option<RGB>> = Vec::new();
        for _ in 0..number_of_functions {
            let hsv = HSV(rng.gen(), 1., 1.);
            colors.push(Some(hsv.to_rgb()));
        }

        let sqrt3by4 = (3. as Real).sqrt()/4.;
        let transformations = vec![
            Transformation::affine(-1./4., sqrt3by4, 1./4.,
                                   -sqrt3by4, -1./4., sqrt3by4),
            Transformation::affine(1./2., 0., 1./4.,
                                   0., 1./2., sqrt3by4),
            Transformation::affine(-1./4., -sqrt3by4, 1.,
                                   sqrt3by4, -1./4., 0.),
        ];

        let description = "Sierpinski Gasket".to_owned();

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
