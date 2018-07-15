use std::f64::consts::PI;

use fractal::FractalBuilder;

use super::{Generic, Lrules};

impl FractalBuilder
{
    pub fn gosper_curve(self) -> Generic {
        let iterations = match self.iterations {
            Some(n) => n,
            None => 6
        };
        let description = format!("Gosper curve, n = {}", iterations);

        info!("Will render {}", description);

        Generic {
            description,
            iterations,
            rules: Lrules::from_string("A", "F → /, A → AF-BF--BF+AF++AFAF+BF-, B → +AF-BFBF--BF-AF++AF+BF"),
            angle: PI/3.,
        }
    }
}
