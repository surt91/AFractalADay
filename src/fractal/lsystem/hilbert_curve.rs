use fractal::FractalBuilder;

use super::LSystem;
use super::turtle::{Turtle, Canvas};

extern crate rayon;
use self::rayon::prelude::*;

pub struct HilbertCurve {

}

enum Alphabet {
    F,
    L,
    R,
    P,
    M,
}

impl LSystem for HilbertCurve {
    fn description(&self) -> &str {
        "Hilbert Curve"
    }

    fn get_canvas(&self) -> Canvas {
        let mut canvas = Canvas::new();

        // variables : L R
        // constants : + −
        // start  : L
        // rules  : (L → +RF-LFL-FR+), (R → -LF+RFR+FL-)

        let mut state = vec![Alphabet::L];

        let n = 5;
        for _ in 0..n {
            state = state.par_iter()
                .map(|i|
                    match i {
                        &Alphabet::L => vec![
                            Alphabet::P,
                            Alphabet::R,
                            Alphabet::F,
                            Alphabet::M,
                            Alphabet::L,
                            Alphabet::F,
                            Alphabet::L,
                            Alphabet::M,
                            Alphabet::F,
                            Alphabet::R,
                            Alphabet::P,
                        ],
                        &Alphabet::R => vec![
                            Alphabet::M,
                            Alphabet::L,
                            Alphabet::F,
                            Alphabet::P,
                            Alphabet::R,
                            Alphabet::F,
                            Alphabet::R,
                            Alphabet::P,
                            Alphabet::F,
                            Alphabet::L,
                            Alphabet::M,
                        ],
                        &Alphabet::F => vec![Alphabet::F],
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
                _ => (),
            };
        }

        canvas
    }
}

impl FractalBuilder
{
    pub fn hilbert_curve(self) -> HilbertCurve {
        HilbertCurve {}
    }
}
