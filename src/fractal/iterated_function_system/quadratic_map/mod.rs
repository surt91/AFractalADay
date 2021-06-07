mod random_map;
mod sampler;

use serde::{self, Serialize, Deserialize};
use rand::{Rng, SeedableRng};

use super::{IteratedFunctionSystem, SuggestedIterations};
use sampler::QuadraticMapSampler;
use crate::{color::RGB, histogram::BoundsTypes};
use crate::numbers::Real;

use super::{Perturbable, Samplable};

use super::{IterationFractalType, RngType, default_rng};


fn default_gamma() -> f64 {
    4.0
}

fn default_vibrancy() -> f64 {
    0.5
}

fn default_bounds() -> BoundsTypes {
    BoundsTypes::StrictBounds
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuadraticMap
{
    #[serde(skip)]
    #[serde(default = "default_rng")]
    rng: RngType,
    pub description: String,
    pub color: RGB,
    pub a: Vec<Real>,
    #[serde(default = "default_bounds")]
    pub bounds: BoundsTypes,
    #[serde(default = "default_gamma")]
    pub gamma: f64,
    #[serde(default = "default_vibrancy")]
    pub vibrancy: f64
}

impl QuadraticMap {
    fn from_string(s: &str) -> Vec<f64> {
        assert!(s.len() == 12);
        let mut a: Vec<f64> = Vec::new();
        for c in s.chars() {
            assert!(c.is_ascii_alphabetic());
            let tmp = (c.to_ascii_lowercase() as u8 - 'a' as u8) as f64;
            a.push(0.1*tmp-1.2);
        }
        a
    }
}

impl IteratedFunctionSystem for QuadraticMap
{
    fn needs_strict_bounds(&self) -> BoundsTypes {
        self.bounds
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

    fn suggested_iterations_draft(&self) -> SuggestedIterations {
        SuggestedIterations::PerPixel(5)
    }

    fn get_rng(&mut self) -> &mut RngType
    {
        &mut self.rng
    }

    fn get_sampler(&mut self) -> Box<dyn Samplable + Send> {
        let rng = RngType::seed_from_u64(self.rng.gen::<u64>());

        let p = [self.rng.gen_range(-0.5, 0.5), self.rng.gen_range(-0.5, 0.5)];

        Box::new(QuadraticMapSampler {
            rng,
            color: self.color.clone(),
            a: self.a.clone(),
            p,
        })
    }

    fn get_serializable(&self) -> IterationFractalType {
        IterationFractalType::QuadraticMap(self.clone())
    }
}
