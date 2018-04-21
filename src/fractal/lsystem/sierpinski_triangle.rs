use std::f64::consts::PI;

use fractal::FractalBuilder;

use super::LSystem;
use super::turtle::{Turtle, Canvas};

extern crate rayon;
use self::rayon::prelude::*;

pub struct KochCurve {

}

enum Alphabet {
    A,
    B,
    P,
    M,
}

impl LSystem for KochCurve {
    fn description(&self) -> &str {
        "Koch Curve"
    }

    fn get_canvas(&self) -> Canvas {
        let mut canvas = Canvas::new();

        // variables : A B
        // constants : + −
        // start  : A
        // rules  : (A → B−A−B), (B → A+B+A)
        // angle  : 60°

        let mut state = vec![Alphabet::A];

        let n = 6;
        for _ in 0..n {
            state = state.par_iter()
                .map(|i|
                    match i {
                        &Alphabet::A => vec![
                            Alphabet::B,
                            Alphabet::M,
                            Alphabet::A,
                            Alphabet::M,
                            Alphabet::B,
                        ],
                        &Alphabet::B => vec![
                            Alphabet::A,
                            Alphabet::P,
                            Alphabet::B,
                            Alphabet::P,
                            Alphabet::A,
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
                Alphabet::A | Alphabet::B => canvas.forward(1.),
                Alphabet::P => canvas.turn(PI/3.),
                Alphabet::M => canvas.turn(-PI/3.),
            };
        }

        canvas
    }
}

impl FractalBuilder
{
    pub fn sierpinski_triangle(self) -> KochCurve {
        KochCurve {}
    }
}
