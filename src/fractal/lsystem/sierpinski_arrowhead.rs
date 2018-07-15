use std::f64::consts::PI;

use fractal::FractalBuilder;

use super::{Generic, Lrules};


impl FractalBuilder
{
    pub fn sierpinski_arrowhead(self) -> Generic {
        let iterations = match self.iterations {
            Some(n) => n,
            None => 6
        };
        let description = format!("Sierpinski arrowhead, n = {}", iterations);

        info!("Will render {}", description);

        Generic {
            description,
            iterations,
            rules: Lrules::from_string("A", "F → /, A → BF-AF-BF, B → AF+BF+AF"),
            angle: PI/3.,
        }
    }
}
