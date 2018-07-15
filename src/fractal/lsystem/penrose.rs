use std::f64::consts::PI;

use fractal::FractalBuilder;

use super::{Generic, Lrules};


impl FractalBuilder
{
    pub fn penrose_tiling(self) -> Generic {
        let iterations = match self.iterations {
            Some(n) => n,
            None => 6
        };
        let description = format!("Penrose tiling, n = {}", iterations);

        info!("Will render {}", description);

        Generic {
            description,
            iterations,
            rules: Lrules::from_string(
                        "+WF--XF---YF--ZF",
                        "F → /,
                         W → YF++ZF----XF[-YF----WF]++,
                         X → +YF--ZF[---WF--XF]+,
                         Y → -WF++XF[+++YF++ZF]-,
                         Z → --YF++++WF[+ZF++++XF]--XF"
            ),
            angle: PI/5.
        }
    }
}
