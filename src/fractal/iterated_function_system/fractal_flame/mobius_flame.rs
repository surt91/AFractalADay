extern crate rand;
use self::rand::Rng;

use itertools;

use color::{HSV, RGB};
use super::{Transformation, MobiusTransformation, AffineTransformation, NonlinearTransformation, FractalFlame, Symmetry};
use fractal::FractalBuilder;
use super::RngType;
use super::Variation;

use numbers::Real;
use std::f64::consts::PI as PI_;
const PI: Real = PI_ as Real;

impl FractalBuilder
{
    pub fn mobius_flame(self) -> FractalFlame<RngType> {
        let mut rng = self.seed_rng();

        let number_of_functions = rng.gen_range(2, 7);

        let prob: Vec<f64> = rng.gen_iter().take(number_of_functions).collect();
        let mut p = 0.;
        let p_norm: f64 = prob.iter().sum();
        let mut probabilities: Vec<f64> = Vec::new();
        for i in prob {
            p += i/p_norm;
            probabilities.push(p);
        }

        let mut colors: Vec<RGB> = Vec::new();
        for _ in 0..number_of_functions {
            let hsv = HSV(rng.gen(), 1., 1.);
            colors.push(hsv.to_rgb());
        }
        let mut transformations: Vec<Transformation> =
                itertools::repeat_call(|| Transformation::Mobius(MobiusTransformation::random(&mut rng)))
                          .take(number_of_functions)
                          .collect();

        let variation = match self.variation {
            Some(v) => NonlinearTransformation::new(v),
            None => NonlinearTransformation::new(Variation::Linear)
        };

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
            // black will be treated as transparent
            // FIXME: .make colors Vec<Option<RGB>>
            let hsv = HSV(0., 0., 0.);
            colors.push(hsv.to_rgb());
        }

        let description = format!("Möbius Flame: '{}' Variation, {} Möbius transformations with {}",
                                   variation.name(),
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

        let number_of_functions = number_of_functions + number_of_symmetries - 1;

        FractalFlame {
            rng,
            description,
            number_of_functions,
            probabilities,
            colors,
            transformations,
            variation,
            strict_bounds: false
        }
    }
}