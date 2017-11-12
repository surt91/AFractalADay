extern crate std;
extern crate num;

extern crate rand;
use self::rand::{Rng, SeedableRng};

use super::variation::Variation;
use super::{RngType, SeedType};

#[derive(Default)]
pub struct IteratedFunctionSystemBuilder {
    pub seed: Option<SeedType>,
    pub variation: Option<Variation>,
}

// Builder Pattern
impl IteratedFunctionSystemBuilder
{
    pub fn seed_rng(&self) -> RngType {
        match self.seed {
            Some(x) => RngType::from_seed(&x),
            None => RngType::from_seed(&rand::weak_rng().gen::<SeedType>())
        }
    }

    pub fn new() -> IteratedFunctionSystemBuilder {
        IteratedFunctionSystemBuilder {
            seed: None,
            variation: None,
        }
    }

    pub fn seed(mut self, seed: usize) -> IteratedFunctionSystemBuilder {
        let s = [seed];
        self.seed = Some(rand::StdRng::from_seed(&s).gen::<SeedType>());
        self
    }

    pub fn variation(mut self, variation: &Variation) -> IteratedFunctionSystemBuilder {
        self.variation = Some(variation.clone());
        self
    }
}
