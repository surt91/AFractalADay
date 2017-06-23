extern crate std;
extern crate num;

extern crate rand;

#[derive(Default)]
pub struct IteratedFunctionSystemBuilder {
    pub seed: Option<usize>
}

// Builder Pattern to create a NewtonFractal
impl IteratedFunctionSystemBuilder {
    pub fn new() -> IteratedFunctionSystemBuilder {
        IteratedFunctionSystemBuilder {
            seed: None,
        }
    }

    pub fn seed(mut self, seed: usize) -> IteratedFunctionSystemBuilder {
        self.seed = Some(seed);
        self
    }
}
