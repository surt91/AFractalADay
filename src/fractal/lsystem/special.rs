use std::f64::consts::PI;

use crate::fractal::FractalBuilder;

use super::Generic;


impl FractalBuilder
{
    pub fn bush(&self) -> Generic {
        Generic::from_rules(
            "Bush",
            "F",
            "F → FF[--F+F+F][+F-F-F]",
            PI/6.,
            self.iterations,
        )
    }

    pub fn gosper_curve(&self) -> Generic {
        Generic::from_rules(
            &format!("Gosper curve"),
            "A",
            "F → /, A → AF-BF--BF+AF++AFAF+BF-, B → +AF-BFBF--BF-AF++AF+BF",
            PI/3.,
            self.iterations,
        )
    }

    pub fn hilbert_curve(&self) -> Generic {
        Generic::from_rules(
            &format!("Hilbert curve"),
            "L",
            "L → +RF-LFL-FR+, R → -LF+RFR+FL-",
            PI/2.,
            self.iterations,
        )
    }

    pub fn koch_curve(&self) -> Generic {
        Generic::from_rules(
            &format!("Koch curve"),
            "F",
            "F → F+F−F−F+F",
            PI/2.,
            self.iterations,
        )
    }

    pub fn penrose_tiling(&self) -> Generic {
        Generic::from_rules(
            &format!("Penrose tiling"),
            "+WF--XF---YF--ZF",
            "F → /,
             W → YF++ZF----XF[-YF----WF]++,
             X → +YF--ZF[---WF--XF]+,
             Y → -WF++XF[+++YF++ZF]-,
             Z → --YF++++WF[+ZF++++XF]--XF",
            PI/5.,
            self.iterations,
        )
    }

    pub fn sierpinski_arrowhead(&self) -> Generic {
        Generic::from_rules(
            &format!("Sierpinski arrowhead"),
            "A",
            "F → /, A → BF-AF-BF, B → AF+BF+AF",
            PI/3.,
            self.iterations,
        )
    }

    pub fn pentigree(&self) -> Generic {
        Generic::from_rules(
            &format!("Pentigree"),
            "F-F-F-F-F",
            "F → F-F++F+F-F-F",
            2.*PI/5.,
            self.iterations,
        )
    }

    pub fn tritile(&self) -> Generic {
        Generic::from_rules(
            &format!("Tritile"),
            "OFF",
            "F → F-FF-, O → -[OFF]-",
            PI/3.,
            self.iterations,
        )
    }

    pub fn ldragon(&self) -> Generic {
        Generic::from_rules(
            &format!("Dragon curve"),
            "FX",
            "X → X+YF+, Y → -FX-Y",
            PI/2.,
            self.iterations,
        )
    }
}
