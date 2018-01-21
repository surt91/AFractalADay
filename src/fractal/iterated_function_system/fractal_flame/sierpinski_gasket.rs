extern crate rand;
use self::rand::Rng;

use color::{HSV, RGB};
use super::{Transformation, NonlinearTransformation, Variation, FractalFlame};
use fractal::FractalBuilder;
use super::RngType;

use numbers::Real;

impl FractalBuilder
{
    pub fn sierpinski_gasket(self) -> FractalFlame<RngType> {
        let mut rng = self.seed_rng();

        let number_of_functions = 3;
        let probabilities = vec![0.33, 0.66, 1.];

        let mut colors: Vec<RGB> = Vec::new();
        for _ in 0..number_of_functions {
            let hsv = HSV(rng.gen(), 1., 1.);
            colors.push(hsv.to_rgb());
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

        let mut description = "Sierpinski Gasket".to_owned();

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
