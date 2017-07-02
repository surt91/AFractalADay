extern crate rand;
use self::rand::Rng;

use color::{HSV, RGB};
use super::{AffineTransformation, NonlinearTransformation, Variation, FractalFlame};
use super::IteratedFunctionSystemBuilder;

use numbers::Real;

use std::f64::consts::PI as PI_;
const PI: Real = PI_ as Real;
// const R: Real = (3. - (5. as Real).sqrt())/2.;
const R: Real = (3. - 2.23606797749979)/2.;

impl IteratedFunctionSystemBuilder {
    pub fn sierpinski_pentagon(self) -> FractalFlame {
        let mut rng: rand::StdRng = match self.seed {
            Some(x) => { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) },
            None => rand::StdRng::new().unwrap()
        };

        let seed = match self.seed {
            Some(x) => x,
            None => 1
        };

        let number_of_functions = 5;
        let probabilities = vec![0.2, 0.4, 0.6, 0.8, 1.];

        let mut colors: Vec<RGB> = Vec::new();
        for _ in 0..number_of_functions {
            let hsv = HSV(rng.gen(), 1., 1.);
            colors.push(hsv.to_rgb());
        }

        let x3 = R * (1. + (72.*PI/180.).cos() + (36.*PI/180.).cos());
        let y3 = R * ((72.*PI/180.).sin() + (36.*PI/180.).sin());
        let x4 = R * (36.*PI/180.).cos();
        let y4 = R * (2.*(72.*PI/180.).sin() + (36.*PI/180.).sin());
        let x5 = R * (-(72.*PI/180.).cos() + (36.*PI/180.).cos() - 1.);
        let affine_transformations = vec![
            AffineTransformation::new(R, 0., 0.,
                                      0., R, 0.),
            AffineTransformation::new(R, 0., 1.-R,
                                      0., R, 0.),
            AffineTransformation::new(R, 0., x3,
                                      0., R, y3),
            AffineTransformation::new(R, 0., x4,
                                      0., R, y4),
            AffineTransformation::new(R, 0., x5,
                                      0., R, y3),
        ];

        let mut description = "Sierpinski Pentagon".to_owned();

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
