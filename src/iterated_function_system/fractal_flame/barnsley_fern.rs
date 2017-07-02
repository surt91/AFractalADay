extern crate rand;

use color::RGB;
use super::{AffineTransformation, NonlinearTransformation, Variation, FractalFlame};
use super::IteratedFunctionSystemBuilder;

impl IteratedFunctionSystemBuilder {
    pub fn barnsley_fern(self) -> FractalFlame {
        let rng: rand::StdRng = match self.seed {
            Some(x) => { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) },
            None => rand::StdRng::new().unwrap()
        };

        let seed = match self.seed {
            Some(x) => x,
            None => 1
        };

        let number_of_functions = 4;
        let probabilities = vec![0.01, 0.86, 0.93, 1.];

        let colors = vec![
            RGB(0.5, 0.5, 0.1),
            RGB(0.02, 0.6, 0.001),
            RGB(0.8, 0.8, 0.),
            RGB(0.8, 0.6, 0.)
        ];
        let affine_transformations = vec![
            AffineTransformation::new(0., 0., 0., 0., 0.16, 0.),
            AffineTransformation::new(0.85, 0.04, 0., -0.04, 0.85, 1.6),
            AffineTransformation::new(0.2, -0.26, 0., 0.23, 0.22, 1.6),
            AffineTransformation::new(-0.15, 0.28, 0., 0.26, 0.24, 0.44),
        ];

        let mut description = "Barnsley Fern".to_owned();

        let nonlinear_transformation = match self.variation {
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
        debug!("affine transformations : {:?}", affine_transformations);
        debug!("Variation              : {:?}", nonlinear_transformation);

        FractalFlame {
            rng,
            description,
            seed,
            number_of_functions,
            probabilities,
            colors,
            affine_transformations,
            nonlinear_transformation,
            strict_bounds: true
        }
    }
}
