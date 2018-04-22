use std::f64::consts::PI;

use fractal::FractalBuilder;

use super::LSystem;
use super::turtle::{Turtle, Canvas};

extern crate rayon;
use self::rayon::prelude::*;

pub struct Bush {
    iterations: u32,
    description: String,
}

enum Alphabet {
    F,
    BL,
    BR,
    P,
    M,
}

impl LSystem for Bush {
    fn description(&self) -> &str {
        &self.description
    }

    fn get_canvas(&self) -> Canvas {
        let mut canvas = Canvas::new();

        // variables : F
        // constants : + − [ ]
        // start  : F
        // rules  : (F → FF[--F+F+F][+F-F-F])

        let mut state = vec![Alphabet::F];

        for _ in 0..self.iterations {
            state = state.par_iter()
                .map(|i|
                    match i {
                        &Alphabet::F => vec![
                            Alphabet::F,
                            Alphabet::F,
                            Alphabet::BL,
                            Alphabet::M,
                            Alphabet::M,
                            Alphabet::F,
                            Alphabet::P,
                            Alphabet::F,
                            Alphabet::P,
                            Alphabet::F,
                            Alphabet::BR,
                            Alphabet::BL,
                            Alphabet::P,
                            Alphabet::F,
                            Alphabet::M,
                            Alphabet::F,
                            Alphabet::M,
                            Alphabet::F,
                            Alphabet::BR,
                        ],
                        &Alphabet::P => vec![Alphabet::P],
                        &Alphabet::M => vec![Alphabet::M],
                        &Alphabet::BL => vec![Alphabet::BL],
                        &Alphabet::BR => vec![Alphabet::BR],
                    }
                )
                .flatten()
                .collect();
        }

        for i in state {
            match i {
                Alphabet::F => canvas.forward(1.),
                Alphabet::P => canvas.turn(PI/6.),
                Alphabet::M => canvas.turn(-PI/6.),
                Alphabet::BL => canvas.push(),
                Alphabet::BR => canvas.pop(),
            };
        }

        canvas
    }
}

impl FractalBuilder
{
    pub fn bush(self) -> Bush {
        let iterations = match self.iterations {
            Some(n) => n,
            None => 6
        };
        let description = format!("Bush, n = {}", iterations);

        info!("Will render {}", description);

        Bush {
            description,
            iterations,
        }
    }
}
