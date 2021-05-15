mod barnsley_fern;
mod heighway_dragon;
mod sierpinski_gasket;
mod sierpinski_pentagon;
mod pythagorean_tree;
mod appolonian_gasket;
mod mobius_flame;
mod fractal_flame;

mod from_json;

use serde::{self, Serialize, Deserialize};
use rand::{Rng, SeedableRng};

use super::IteratedFunctionSystem;
use super::IteratedFunctionSystemSampler;
use crate::fractal::Symmetry;
use super::{Transformation,NonlinearTransformation,AffineTransformation,MobiusTransformation};
use crate::color::RGB;

use super::{Samplable};
use super::{IterationFractalType, RngType, default_rng};


fn default_bounds() -> bool {
    false
}

fn default_gamma() -> f64 {
    4.0
}

fn default_vibrancy() -> f64 {
    0.5
}

fn default_post_transform() -> Transformation {
    Transformation::identity()
}

fn default_final_transform() -> NonlinearTransformation {
    NonlinearTransformation::identity()
}

fn default_final_color() -> Option<RGB> {
    None
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FractalFlame
{
    #[serde(skip)]
    #[serde(default = "default_rng")]
    rng: RngType,
    pub description: String,
    pub probabilities: Vec<f64>,
    pub colors: Vec<Option<RGB>>,
    pub transformations: Vec<Transformation>,
    pub variation: NonlinearTransformation,
    #[serde(default = "default_post_transform")]
    pub post_transform: Transformation,
    #[serde(default = "default_final_transform")]
    pub final_transform: NonlinearTransformation,
    #[serde(default = "default_final_color")]
    pub final_color: Option<RGB>,
    #[serde(default = "default_bounds")]
    pub strict_bounds: bool,
    #[serde(default = "default_gamma")]
    pub gamma: f64,
    #[serde(default = "default_vibrancy")]
    pub vibrancy: f64
}

impl IteratedFunctionSystem for FractalFlame
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

    fn get_sampler(&mut self) -> Box<dyn Samplable + Send> {
        let rng = RngType::seed_from_u64(self.rng.gen::<u64>());

        let p = [0.05, 0.05];
        let rgb = RGB(0., 0., 0.);

        Box::new(IteratedFunctionSystemSampler {
            rng,
            number_of_functions: self.probabilities.len(),
            probabilities: self.probabilities.clone(),
            colors: self.colors.clone(),
            transformations: self.transformations.clone(),
            variation: self.variation.clone(),
            post_transform: self.post_transform.clone(),
            final_transform: self.final_transform.clone(),
            final_color: self.final_color.clone(),
            p,
            rgb
        })
    }

    fn get_serializable(&self) -> IterationFractalType {
        IterationFractalType::IFS(self.clone())
    }
}
