extern crate std;
extern crate num;

extern crate rand;

#[derive(Default)]
pub struct ColoredIFSBuilder {
    pub seed: Option<usize>
}

// Builder Pattern
impl ColoredIFSBuilder {
    pub fn new() -> ColoredIFSBuilder {
        ColoredIFSBuilder {
            seed: None,
        }
    }

    pub fn seed(mut self, seed: usize) -> ColoredIFSBuilder {
        self.seed = Some(seed);
        self
    }
}
