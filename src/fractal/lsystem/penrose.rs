use std::f64::consts::PI;

use fractal::FractalBuilder;

use super::LSystem;
use super::turtle::{Turtle, Canvas};

extern crate rayon;
use self::rayon::prelude::*;

pub struct PenroseTiling {
    iterations: u32,
    description: String,
}

enum Alphabet {
    F,
    X,
    Y,
    Z,
    W,
    BL,
    BR,
    P,
    M,
}

fn parse(rule: &str) -> Vec<Alphabet> {
    rule.chars()
        .map(|c| match c {
            'F' => Alphabet::F,
            'X' => Alphabet::X,
            'Y' => Alphabet::Y,
            'Z' => Alphabet::Z,
            'W' => Alphabet::W,
            '[' => Alphabet::BL,
            ']' => Alphabet::BR,
            '+' => Alphabet::P,
            '-' => Alphabet::M,
            _ => unreachable!()
        })
        .collect()

}

impl LSystem for PenroseTiling {
    fn description(&self) -> &str {
        &self.description
    }

    fn get_canvas(&self) -> Canvas {
        let mut canvas = Canvas::new();

        let mut state = parse("+WF--XF---YF--ZF");

        for _ in 0..self.iterations {
            state = state.par_iter()
                .map(|i|
                    match i {
                        &Alphabet::F => Vec::new(),
                        &Alphabet::W => parse("YF++ZF----XF[-YF----WF]++"),
                        &Alphabet::X => parse("+YF--ZF[---WF--XF]+"),
                        &Alphabet::Y => parse("-WF++XF[+++YF++ZF]-"),
                        &Alphabet::Z => parse("--YF++++WF[+ZF++++XF]--XF"),
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
                Alphabet::P => canvas.turn(PI/5.),
                Alphabet::M => canvas.turn(-PI/5.),
                Alphabet::BL => canvas.push(),
                Alphabet::BR => canvas.pop(),
                _ => ()
            };
        }

        canvas
    }
}

impl FractalBuilder
{
    pub fn penrose_tiling(self) -> PenroseTiling {
        let iterations = match self.iterations {
            Some(n) => n,
            None => 6
        };
        let description = format!("Penrose tiling, n = {}", iterations);

        info!("Will render {}", description);

        PenroseTiling {
            description,
            iterations,
        }
    }
}
