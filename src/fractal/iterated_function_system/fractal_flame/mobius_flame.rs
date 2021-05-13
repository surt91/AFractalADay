use log::{debug, info};
use rand::Rng;
use rand::distributions::Standard;

use std::iter;

use crate::color::{HSV, RGB};
use super::{Transformation, MobiusTransformation, AffineTransformation, NonlinearTransformation, FractalFlame, Symmetry};
use crate::fractal::FractalBuilder;

use crate::numbers::Real;
use std::f64::consts::PI as PI_;
const PI: Real = PI_ as Real;

impl FractalBuilder
{
    pub fn mobius_flame(self) -> FractalFlame {
        let mut rng = self.seed_rng();

        let number_of_functions = rng.gen_range(2, 7);

        let prob: Vec<f64> = rng.sample_iter(&Standard).take(number_of_functions).collect();
        let mut p = 0.;
        let p_norm: f64 = prob.iter().sum();
        let mut probabilities: Vec<f64> = Vec::new();
        for i in prob {
            p += i/p_norm;
            probabilities.push(p);
        }

        let mut colors: Vec<Option<RGB>> = Vec::new();
        for _ in 0..number_of_functions {
            let hsv = HSV(rng.gen(), 1., 1.);
            colors.push(Some(hsv.to_rgb()));
        }
        let mut transformations: Vec<Transformation> =
            iter::repeat_with(|| Transformation::Mobius(MobiusTransformation::random(&mut rng)))
                .take(number_of_functions)
                .collect();

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

        let strict_bounds = rng.gen();

        // handle symmetries
        let symmetry = match self.symmetry {
            Some(s) => s,
            None => Symmetry::random(&mut rng)
        };

        let number_of_symmetries: usize = match symmetry {
            Symmetry::None => 1,
            Symmetry::Vertical => {
                transformations.push(
                    Transformation::Affine(
                        AffineTransformation::vertical_mirror()
                    )
                );
                2
            }
            Symmetry::Horizontal => {
                transformations.push(
                    Transformation::Affine(
                        AffineTransformation::horizontal_mirror()
                    )
                );
                2
            }
            Symmetry::Rotational(x) => {
                for i in 1..x {
                    transformations.push(
                        Transformation::Affine(
                            AffineTransformation::rotate(2.*PI/x as Real * i as Real)
                        )
                    );
                }
                x
            }
        };

        for i in 0..probabilities.len() {
            probabilities[i] /= number_of_symmetries as f64;
        }

        p = 1./number_of_symmetries as f64;
        for _ in 1..number_of_symmetries {
            p += 1./number_of_symmetries as f64;
            probabilities.push(p);
            colors.push(None);
        }

        let description = format!("Möbius Flame: {} Möbius transformations with {}",
                                   number_of_functions,
                                   symmetry
                                 );

        info!("Will render {}", description);

        debug!("number of functions    : {:?}", number_of_functions);
        debug!("cumulative probabilites: {:?}", probabilities);
        debug!("colors                 : {:?}", colors);
        debug!("mobius transformations : {:?}", transformations);
        debug!("Variation              : {:?}", variation);
        debug!("Symmetry               : {:?}", symmetry);

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
            strict_bounds,
            gamma,
            vibrancy,
        }
    }
}
