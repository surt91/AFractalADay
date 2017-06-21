extern crate std;
extern crate num;

extern crate rand;

use numbers::{Coef, Formula};

pub struct IteratedFunctionSystemBuilder {
    pub seed: Option<usize>,
    pub iterations: Option<usize>
}

// Builder Pattern to create a NewtonFractal
impl IteratedFunctionSystemBuilder {
    pub fn new() -> IteratedFunctionSystemBuilder {
        IteratedFunctionSystemBuilder {
            seed: None,
            iterations: None,
        }
    }

    pub fn seed(mut self, seed: usize) -> IteratedFunctionSystemBuilder {
        self.seed = Some(seed);
        self
    }

    pub fn iterations(mut self, iterations: usize) -> IteratedFunctionSystemBuilder {
        self.iterations = Some(iterations);
        self
    }
}
