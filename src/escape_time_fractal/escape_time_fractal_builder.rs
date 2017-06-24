extern crate std;
extern crate num;

extern crate rand;

use numbers::{Coef, Formula};

use super::style::Style;

#[derive(Default)]
pub struct EscapeTimeFractalBuilder {
    pub a: Option<Coef>,
    pub f: Option<Formula>,
    pub seed: Option<usize>,
    pub style: Option<Style>
}

// Builder Pattern
impl EscapeTimeFractalBuilder {
    pub fn new() -> EscapeTimeFractalBuilder {
        EscapeTimeFractalBuilder {
            a: None,
            f: None,
            seed: None,
            style: None
        }
    }

    pub fn coefficient(mut self, a: Coef) -> EscapeTimeFractalBuilder {
        self.a = Some(a);
        self
    }

    pub fn formula(mut self, f: Formula) -> EscapeTimeFractalBuilder {
        self.f = Some(f);
        self
    }

    pub fn seed(mut self, seed: usize) -> EscapeTimeFractalBuilder {
        self.seed = Some(seed);
        self
    }

    pub fn style(mut self, style: Style) -> EscapeTimeFractalBuilder {
        self.style = Some(style);
        self
    }
}
