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

use rand::{Rng, SeedableRng};

use super::IteratedFunctionSystem;
use super::IteratedFunctionSystemConfig;
use super::IteratedFunctionSystemSampler;
use fractal::Symmetry;
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
    post_transform: Transformation,
    final_transform: NonlinearTransformation,
    final_color: Option<RGB>,
    strict_bounds: bool,
    gamma: f64,
    vibrancy: f64
}

impl IteratedFunctionSystem for FractalFlame<RngType>
{
    fn needs_strict_bounds(&self) -> bool {
        self.strict_bounds
    }

    fn gamma(&self) -> f64 {
        self.gamma
    }

    fn vibrancy(&self) -> f64 {
        self.vibrancy
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn get_rng(&mut self) -> &mut RngType
    {
        &mut self.rng
    }

    fn get_sampler(&mut self) -> IteratedFunctionSystemSampler<RngType> {
        // let s = self.rng.gen::<SeedType>();
        let rng = RngType::from_seed(&self.rng.gen::<SeedType>());

        let p = [0.05, 0.05];
        let r = 0.;
        let g = 0.;
        let b = 0.;

        IteratedFunctionSystemSampler {
            rng,
            number_of_functions: self.number_of_functions,
            probabilities: self.probabilities.clone(),
            colors: self.colors.clone(),
            transformations: self.transformations.clone(),
            variation: self.variation.clone(),
            post_transform: self.post_transform.clone(),
            final_transform: self.final_transform.clone(),
            final_color: self.final_color.clone(),
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
            post_transform: self.post_transform.clone(),
            final_transform: self.final_transform.clone(),
            final_color: self.final_color.clone(),
            description: self.description().to_owned(),
            strict_bounds: self.strict_bounds.clone(),
            gamma: self.gamma.clone(),
            vibrancy: self.vibrancy.clone()
        }
    }
}
