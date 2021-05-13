use serde_json;

use log::info;

use std::f64::consts::PI;

use itertools::Itertools;
use serde::{self, Serialize, Deserialize};

use crate::fractal::FractalBuilder;

use super::LSystem;
use super::turtle::{Turtle, Canvas};
use super::Alphabet;
use super::Lrules;
use super::rules::{lrule_serialize,lrule_deserialize};

use rayon::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Generic {
    pub iterations: u32,
    pub description: String,
    #[serde(serialize_with = "lrule_serialize", deserialize_with = "lrule_deserialize")]
    pub rules: Lrules,
    pub angle: f64,
}

impl Generic {
    pub fn from_rules(
        description: &str,
        start: &str,
        rules: &str,
        angle: f64,
        iterations: Option<u32>
    ) -> Generic {
        let iterations = match iterations {
            Some(n) => n,
            None => 6
        };

        info!("Will render {}, n = {}", description, iterations);

        Generic {
            description: description.to_owned(),
            iterations,
            rules: Lrules::from_string(start, rules),
            angle,
        }
    }
}

impl LSystem for Generic {
    fn description(&self) -> &str {
        &self.description
    }

    fn get_serializable(&self) -> Generic {
        self.clone()
    }

    fn get_canvas(&self) -> Canvas {
        let mut canvas = Canvas::new();

        let mut state = self.rules.start().clone();

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
            Some(s) => s,
            None => Alphabet::parse("F")
        };
        let rules = match self.rules {
            Some(m) => m,
            None => Lrules::random(self.seed)
        };
        let angle = match self.angle {
            Some(f) => f,
            None => 156./180.*PI
        };

        let rule_string = format!("{}", rules);
        let start_string = format!("{}", start.iter().map(|x| format!("{}", x)).join(""));

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
            rules,
            angle
        }
    }

    pub fn lsys_from_json(json: &str) -> Result<Generic, serde_json::Error> {
        serde_json::from_str(json)
    }
}
