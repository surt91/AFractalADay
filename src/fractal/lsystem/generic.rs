use std::f64::consts::PI;

use fractal::FractalBuilder;

use super::LSystem;
use super::turtle::{Turtle, Canvas};
use super::Alphabet;
use super::Lrules;

extern crate rayon;
use self::rayon::prelude::*;


pub struct Generic {
    pub iterations: u32,
    pub description: String,
    pub start: Vec<Alphabet>,
    pub rules: Lrules,
    pub angle: f64,
}

impl LSystem for Generic {
    fn description(&self) -> &str {
        &self.description
    }

    fn get_canvas(&self) -> Canvas {
        let mut canvas = Canvas::new();

        let mut state = self.start.clone();

        for _ in 0..self.iterations {
            state = state.par_iter()
                .map(|i|
                    self.rules[i].clone()
                )
                .flatten()
                .collect();
        }

        for i in state {
            match i {
                Alphabet::F => canvas.forward(1.),
                Alphabet::P => canvas.turn(self.angle),
                Alphabet::M => canvas.turn(-self.angle),
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
    pub fn generic(self) -> Generic {
        let iterations = match self.iterations {
            Some(n) => n,
            None => 5
        };
        let start = match self.start.clone() {
            Some(s) => Alphabet::parse(&s),
            None => Alphabet::parse("F")
        };
        let rules = match self.rules {
            Some(m) => m,
            None => Lrules::random()
        };
        let angle = match self.angle {
            Some(f) => f,
            None => 156./180.*PI
        };

        let rule_string = format!("{}", rules);
        let start_string = format!("{}", self.start.unwrap_or("F".to_string()));

        let description = format!(
            "L-System, start = {}, rules = ({}), angle = {:.0}°, n = {}",
            start_string,
            rule_string,
            angle * 180. / PI,
            iterations,
        );

        info!("Will render {}", description);

        Generic {
            description,
            iterations,
            start,
            rules,
            angle
        }
    }
}
