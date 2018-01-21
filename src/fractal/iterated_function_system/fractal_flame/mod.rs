mod barnsley_fern;
mod heighway_dragon;
mod sierpinski_gasket;
mod sierpinski_pentagon;
mod pythagorean_tree;
mod appolonian_gasket;
mod mobius_flame;
mod fractal_flame;

mod from_json;

extern crate std;
extern crate num;

extern crate rand;
use self::rand::{Rng, SeedableRng};

use numbers::Real;
use super::IteratedFunctionSystem;
use super::IteratedFunctionSystemConfig;
use fractal::{Symmetry,Variation};
use super::{Transformation,NonlinearTransformation,AffineTransformation,MobiusTransformation};
use color::RGB;

use super::{RngType, SeedType};

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
                x.transform(self.p[0], self.p[1])
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

    fn get_serializable(&self) -> IteratedFunctionSystemConfig {
        IteratedFunctionSystemConfig {
            probabilities: self.probabilities.clone(),
            colors: self.colors.clone(),
            transformations: self.transformations.clone(),
            variation: self.variation.clone(),
            description: self.description().to_owned(),
        }
    }
}
