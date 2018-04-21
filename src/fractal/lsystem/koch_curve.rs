use fractal::FractalBuilder;

use super::LSystem;
use super::turtle::{Turtle, Canvas};

extern crate rayon;
use self::rayon::prelude::*;

pub struct KochCurve {

}

enum Alphabet {
    F,
    P,
    M,
}

impl LSystem for KochCurve {
    fn description(&self) -> &str {
        "Koch Curve"
    }

    fn get_canvas(&self) -> Canvas {
        let mut canvas = Canvas::new();

        // variables : F
        // constants : + −
        // start  : F
        // rules  : (F → F+F−F−F+F)

        let mut state = vec![Alphabet::F];

        let n = 5;
        for _ in 0..n {
            state = state.par_iter()
                .map(|i|
                    match i {
                        &Alphabet::F => vec![
                            Alphabet::F,
                            Alphabet::P,
                            Alphabet::F,
                            Alphabet::M,
                            Alphabet::F,
                            Alphabet::M,
                            Alphabet::F,
                            Alphabet::P,
                            Alphabet::F,
                        ],
                        &Alphabet::P => vec![Alphabet::P],
                        &Alphabet::M => vec![Alphabet::M],
                    }
                )
                .flatten()
                .collect();
        }

        for i in state {
            match i {
                Alphabet::F => canvas.forward(1.),
                Alphabet::P => canvas.turn_left(),
                Alphabet::M => canvas.turn_right(),
            };
        }

        canvas
    }
}

impl FractalBuilder
{
    pub fn koch_curve(self) -> KochCurve {
        KochCurve {}
    }
}
