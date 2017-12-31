mod affine_transformation;
use self::affine_transformation::AffineTransformation;
mod mobius_transformation;
use self::mobius_transformation::MobiusTransformation;
mod nonlinear_transformation;
use self::nonlinear_transformation::NonlinearTransformation;
use super::variation::Variation;

mod barnsley_fern;
mod heighway_dragon;
mod sierpinski_gasket;
mod sierpinski_pentagon;
mod pythagorean_tree;

mod two_fractal_flame;
mod rotated_fractal_flame;
mod mobius_flame;
mod three_moebius_flame;

extern crate std;
extern crate num;
use itertools;

extern crate rand;
use self::rand::{Rng, SeedableRng};

use numbers::{Real, Cplx};
use super::IteratedFunctionSystem;
use color::{RGB, HSV};

use super::{RngType, SeedType};
use super::iterated_function_system_builder::IteratedFunctionSystemBuilder;


#[derive(Debug, Clone)]
pub enum Transformation {
    Affine(AffineTransformation),
    Mobius(MobiusTransformation)
}

impl Transformation {
    fn affine(a: Real, b: Real, c: Real, d: Real, e: Real, f: Real) -> Transformation {
        Transformation::Affine(AffineTransformation::new(a, b, c, d, e, f))
    }
    fn mobius(a: Cplx, b: Cplx, c: Cplx, d: Cplx) -> Transformation {
        Transformation::Mobius(MobiusTransformation::new(a, b, c, d))
    }
}

pub struct FractalFlame<T>
    where T: Rng
{
    rng: T,
    pub description: String,
    number_of_functions: usize,
    probabilities: Vec<f64>,
    colors: Vec<RGB>,
    transformations: Vec<Transformation>,
    variation: NonlinearTransformation,
    strict_bounds: bool,
}

pub struct FractalFlameSampler<T>
    where T: Rng
{
    rng: T,
    number_of_functions: usize,
    probabilities: Vec<f64>,
    colors: Vec<RGB>,
    transformations: Vec<Transformation>,
    variation: NonlinearTransformation,
    p: [Real; 2],
    r: f64,
    g: f64,
    b: f64,
}

impl <T> Iterator for FractalFlameSampler<T>
    where T: Rng
{
    type Item = ([Real; 2], RGB);

    fn next(&mut self) -> Option<([Real; 2], RGB)> {
        let r = self.rng.gen::<f64>();

        let mut index = 0;
        for i in 0..self.number_of_functions {
            if r < self.probabilities[i] {
                index = i;
                break;
            }
        }

        let mut is_symmetry_transformation = false;
        let transformed = match self.transformations[index] {
            Transformation::Affine(ref x) => {
                is_symmetry_transformation = x.symmetry;
                x.transform(self.p)
            },
            Transformation::Mobius(ref x) => {
                let z = x.transform(Cplx::new(self.p[0], self.p[1]));
                [z.re, z.im]
            }
        };

        // do not apply variation to symmetry transforms
        if !is_symmetry_transformation {
            self.p = self.variation.transform(transformed);
        } else {
            self.p = transformed;
        }

        let RGB(r, g, b) = self.colors[index];
        // if it is black, ignore it
        // FIXME: better would be Option<RGB>
        if r != 0. || g != 0. || b != 0.
        {
            self.r = (r + self.r)/2.;
            self.g = (g + self.g)/2.;
            self.b = (b + self.b)/2.;
        }

        Some((self.p, RGB(self.r, self.g, self.b)))
    }
}

impl IteratedFunctionSystem for FractalFlame<RngType>
{
    fn needs_strict_bounds(&self) -> bool {
        self.strict_bounds
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn get_rng(&mut self) -> &mut RngType
    {
        &mut self.rng
    }

    fn get_sampler(&mut self) -> FractalFlameSampler<RngType> {
        // let s = self.rng.gen::<SeedType>();
        let rng = RngType::from_seed(&self.rng.gen::<SeedType>());

        let p = [0.05, 0.05];
        let r = 0.;
        let g = 0.;
        let b = 0.;

        FractalFlameSampler {
            rng,
            number_of_functions: self.number_of_functions,
            probabilities: self.probabilities.clone(),
            colors: self.colors.clone(),
            transformations: self.transformations.clone(),
            variation: self.variation.clone(),
            p,
            r,
            g,
            b,
        }
    }
}


impl IteratedFunctionSystemBuilder
{
    pub fn fractal_flame(self) -> FractalFlame<RngType> {
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
        let transformations: Vec<Transformation> =
                itertools::repeat_call(|| Transformation::Affine(AffineTransformation::random(&mut rng)))
                          .take(number_of_functions)
                          .collect();

        let variation = match self.variation {
            Some(v) => NonlinearTransformation::new(v),
            None => NonlinearTransformation::random(&mut rng)
        };

        let description = format!("Fractal Flame: '{}' Variation, {} affine transformations",
                                   variation.name(),
                                   number_of_functions);

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
            strict_bounds: false
        }
    }
}
