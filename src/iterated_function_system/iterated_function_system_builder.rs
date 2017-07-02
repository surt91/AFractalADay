extern crate std;
extern crate num;

extern crate rand;

use super::variation::Variation;

#[derive(Default)]
pub struct IteratedFunctionSystemBuilder {
    pub seed: Option<usize>,
    pub variation: Option<Variation>,
}

// Builder Pattern
impl IteratedFunctionSystemBuilder {
    pub fn new() -> IteratedFunctionSystemBuilder {
        IteratedFunctionSystemBuilder {
            seed: None,
            variation: None,
        }
    }

    pub fn seed(mut self, seed: usize) -> IteratedFunctionSystemBuilder {
        self.seed = Some(seed);
        self
    }

    pub fn variation(mut self, variation: &Variation) -> IteratedFunctionSystemBuilder {
        self.variation = Some(variation.clone());
        self
    }
}
