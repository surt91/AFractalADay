use color::RGB;
use super::{Transformation, NonlinearTransformation, Variation, FractalFlame};
use fractal::{FractalBuilder, RngType};

impl FractalBuilder
{
    pub fn barnsley_fern(self) -> FractalFlame<RngType> {
        let rng = self.seed_rng();

        let number_of_functions = 4;
        let probabilities = vec![0.01, 0.86, 0.93, 1.];

        let colors = vec![
            RGB(0.5, 0.5, 0.1),
            RGB(0.02, 0.6, 0.001),
            RGB(0.8, 0.8, 0.),
            RGB(0.8, 0.6, 0.)
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
            None => NonlinearTransformation::new(Variation::Linear)
        };

        let gamma = match self.gamma {
            Some(s) => s,
            None => 4.
        };

        let vibrancy = match self.vibrancy {
            Some(s) => s,
            None => 0.4
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
            strict_bounds: true,
            gamma,
            vibrancy,
        }
    }
}
