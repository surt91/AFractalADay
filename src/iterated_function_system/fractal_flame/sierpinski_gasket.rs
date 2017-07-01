extern crate rand;
use self::rand::Rng;

use color::{HSV, RGB};
use super::{AffineTransformation, NonlinearTransformation, Variation, FractalFlame};
use super::IteratedFunctionSystemBuilder;

use numbers::Real;

impl IteratedFunctionSystemBuilder {
    pub fn sierpinski_gasket(self) -> FractalFlame {
        let mut rng: rand::StdRng = match self.seed {
            Some(x) => { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) },
            None => rand::StdRng::new().unwrap()
        };

        let seed = match self.seed {
            Some(x) => x,
            None => 1
        };

        let number_of_functions = 3;
        let probabilities = vec![0.33, 0.66, 1.];

        let mut colors: Vec<RGB> = Vec::new();
        for _ in 0..number_of_functions {
            let hsv = HSV(rng.gen(), 1., 1.);
            colors.push(hsv.to_rgb());
        }

        let sqrt3by4 = (3. as Real).sqrt()/4.;
        let affine_transformations = vec![
            AffineTransformation::new(-1./4., sqrt3by4, 1./4.,
                                      -sqrt3by4, -1./4., sqrt3by4),
            AffineTransformation::new(1./2., 0., 1./4.,
                                      0., 1./2., sqrt3by4),
            AffineTransformation::new(-1./4., -sqrt3by4, 1.,
                                      sqrt3by4, -1./4., 0.),
        ];
        let nonlinear_transformation = NonlinearTransformation::new(Variation::Linear);

        let description = "Sierpinski Gasket".to_owned();

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
            nonlinear_transformation
        }
    }
}
