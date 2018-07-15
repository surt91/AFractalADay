use std::f64::consts::PI;

use fractal::FractalBuilder;

use super::{Generic, Lrules};


impl FractalBuilder
{
    pub fn koch_curve(self) -> Generic {
        let iterations = match self.iterations {
            Some(n) => n,
            None => 6
        };
        let description = format!("Koch curve, n = {}", iterations);

        info!("Will render {}", description);

        Generic {
            description,
            iterations,
            rules: Lrules::from_string("F", "F → F+F−F−F+F"),
            angle: PI/2.
        }
    }
}
