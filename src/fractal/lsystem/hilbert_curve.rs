use std::f64::consts::PI;

use fractal::FractalBuilder;

use super::{Generic, Lrules};

impl FractalBuilder
{
    pub fn hilbert_curve(self) -> Generic {
        let iterations = match self.iterations {
            Some(n) => n,
            None => 6
        };
        let description = format!("Hilbert curve, n = {}", iterations);

        info!("Will render {}", description);

        Generic {
            description,
            iterations,
            rules: Lrules::from_string("L", "L → +RF-LFL-FR+, R → -LF+RFR+FL-"),
            angle: PI/2.
        }
    }
}
