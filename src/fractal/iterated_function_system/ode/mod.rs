mod ode_system;
mod sampler;
mod lorenz;

use serde::{self, Serialize, Deserialize};
use rand::{Rng, SeedableRng};
use rand::distributions::{Distribution, Normal};

use super::IteratedFunctionSystem;
use sampler::OdeFractalSampler;
use ode_system::OdeSystem;
use lorenz::LorenzOde;
use crate::{color::RGB, numbers::Real};

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
    Lorenz(LorenzOde),
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
    pub normal: [Real; 3],
    #[serde(default = "default_bounds")]
    pub strict_bounds: bool,
    #[serde(default = "default_gamma")]
    pub gamma: f64,
    #[serde(default = "default_vibrancy")]
    pub vibrancy: f64
}

impl IteratedFunctionSystem for OdeFractal
{
    fn needs_strict_bounds(&self) -> bool {
        self.strict_bounds
    }

    fn suggested_iterations(&self) -> usize {
        10
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

        let ode: Box<dyn OdeSystem> = match &self.ode {
            OdeTypes::Lorenz(x) => Box::new(x.clone()),
        };

        Box::new(OdeFractalSampler {
            rng,
            ode,
            color: self.color.clone(),
            normal: self.normal,
        })
    }

    fn get_serializable(&self) -> IterationFractalType {
        IterationFractalType::OdeFractal(self.clone())
    }
}

fn random_normal(mut rng: &mut impl Rng) -> [Real; 3] {
    let normal = Normal::new(0.0, 1.0);
    let mut coords: Vec<Real> = normal.sample_iter(&mut rng).take(3).collect();
    let norm = coords.iter().map(|x| x*x).sum::<Real>().sqrt();
    coords.iter_mut().for_each(|x| *x /= norm);
    [coords[0], coords[1], coords[2]]
}