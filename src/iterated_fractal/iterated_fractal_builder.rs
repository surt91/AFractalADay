extern crate std;
extern crate num;

extern crate rand;

use numbers::{Coef, Formula};

use iterated_fractal::style::Style;

pub struct IteratedFractalBuilder {
    pub a: Option<Coef>,
    pub f: Option<Formula>,
    pub seed: Option<usize>,
    pub style: Option<Style>
}

// Builder Pattern to create a NewtonFractal
impl IteratedFractalBuilder {
    pub fn new() -> IteratedFractalBuilder {
        IteratedFractalBuilder {
            a: None,
            f: None,
            seed: None,
            style: None
        }
    }

    pub fn coefficient(mut self, a: Coef) -> IteratedFractalBuilder {
        self.a = Some(a);
        self
    }

    pub fn formula(mut self, f: Formula) -> IteratedFractalBuilder {
        self.f = Some(f);
        self
    }

    pub fn seed(mut self, seed: usize) -> IteratedFractalBuilder {
        self.seed = Some(seed);
        self
    }

    pub fn style(mut self, style: Style) -> IteratedFractalBuilder {
        self.style = Some(style);
        self
    }
}
