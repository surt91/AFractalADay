use std::f64::consts::PI;

use fractal::FractalBuilder;

use super::LSystem;
use super::turtle::{Turtle, Canvas};

extern crate rayon;
use self::rayon::prelude::*;

pub struct SierpinskiArrowhead {
    iterations: u32,
    description: String,
}

enum Alphabet {
    A,
    B,
    P,
    M,
}

impl LSystem for SierpinskiArrowhead {
    fn description(&self) -> &str {
        &self.description
    }

    fn get_canvas(&self) -> Canvas {
        let mut canvas = Canvas::new();

        // variables : A B
        // constants : + −
        // start  : A
        // rules  : (A → B−A−B), (B → A+B+A)
        // angle  : 60°

        let mut state = vec![Alphabet::A];

        for _ in 0..self.iterations {
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
    pub fn sierpinski_arrowhead(self) -> SierpinskiArrowhead {
        let iterations = match self.iterations {
            Some(n) => n,
            None => 6
        };
        let description = format!("Sierpinski arrowhead, n = {}", iterations);

        info!("Will render {}", description);

        SierpinskiArrowhead {
            description,
            iterations,
        }
    }
}
