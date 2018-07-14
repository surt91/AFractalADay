use std::f64::consts::PI;

use fractal::FractalBuilder;

use super::{Generic, Alphabet, Lrules};


impl FractalBuilder
{
    pub fn bush(self) -> Generic {
        let iterations = match self.iterations {
            Some(n) => n,
            None => 6
        };
        let description = format!("Bush, n = {}", iterations);

        info!("Will render {}", description);

        Generic {
            description,
            iterations,
            start: Alphabet::parse("F"),
            rules: Lrules::from_string("F → FF[--F+F+F][+F-F-F]"),
            angle: PI/6.
        }
    }
}
