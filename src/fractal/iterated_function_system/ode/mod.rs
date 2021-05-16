mod lorenz;
mod ode_system;
mod sampler;

use serde::{self, Serialize, Deserialize};
use rand::{Rng, SeedableRng};

use super::IteratedFunctionSystem;
use sampler::OdeFractalSampler;
use ode_system::OdeSystem;
use lorenz::LorenzOde;
use crate::color::RGB;
use crate::numbers::Real;

use super::{Perturbable, Samplable};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OdeTypes {
    Lorenz(LorenzOde)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OdeFractal
{
    #[serde(skip)]
    #[serde(default = "default_rng")]
    rng: RngType,
    pub description: String,
    pub color: RGB,
    pub ode: OdeTypes,
    #[serde(default = "default_bounds")]
    pub strict_bounds: bool,
    #[serde(default = "default_gamma")]
    pub gamma: f64,
    #[serde(default = "default_vibrancy")]
    pub vibrancy: f64
}

impl OdeFractal {
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

impl IteratedFunctionSystem for OdeFractal
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

        let ode = match &self.ode {
            OdeTypes::Lorenz(x) => x
        };

        Box::new(OdeFractalSampler {
            rng,
            ode: Box::new(ode.clone()),
            color: self.color.clone(),
        })
    }

    fn get_serializable(&self) -> IterationFractalType {
        IterationFractalType::OdeFractal(self.clone())
    }
}
