use std::f64::consts::PI;

use fractal::FractalBuilder;

use std::collections::HashMap;
use std::ops::Index;

use super::LSystem;
use super::turtle::{Turtle, Canvas};

extern crate rayon;
use self::rayon::prelude::*;

#[derive(Debug)]
pub struct Lrules {
    rules: HashMap<Alphabet, Vec<Alphabet>>
}

impl Lrules {
    fn from_string(s: &str) -> Lrules {
        // TODO implement
        let mut rules: HashMap<Alphabet, Vec<Alphabet>> = HashMap::new();
        // defaults
        rules.insert(Alphabet::Invalid, Vec::new());
        rules.insert(Alphabet::F, vec![Alphabet::F]);
        rules.insert(Alphabet::M, vec![Alphabet::M]);
        rules.insert(Alphabet::P, vec![Alphabet::P]);
        rules.insert(Alphabet::BL, vec![Alphabet::BL]);
        rules.insert(Alphabet::BR, vec![Alphabet::BR]);

        let mut it = s.chars();
        // loop {
            let key = it.next().unwrap();
            it.next(); // jump over :
            let rule = it.take_while(|x| x != &',').collect::<String>();
            rules.insert(Alphabet::new(key), parse(&rule));
        // }

        Lrules{
            rules
        }
    }

    fn random() -> Lrules {
        Lrules::from_string("F:--FFF+")
    }
}

impl<'a> Index<&'a Alphabet> for Lrules {
    type Output = Vec<Alphabet>;

    fn index(&self, x: &'a Alphabet) -> &Vec<Alphabet> {
        if !self.rules.contains_key(x) {
            &self.rules[&Alphabet::Invalid]
        } else {
            &self.rules[x]
        }
    }
}

pub struct Generic {
    iterations: u32,
    description: String,
    start: Vec<Alphabet>,
    rules: Lrules,
    angle: f64,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Alphabet {
    F,
    BL,
    BR,
    P,
    M,
    Marker(char),
    Invalid
}

impl Alphabet {
    fn new(c: char) -> Alphabet {
        match c {
            'F' => Alphabet::F,
            '[' => Alphabet::BL,
            ']' => Alphabet::BR,
            '+' => Alphabet::P,
            '-' => Alphabet::M,
            x => Alphabet::Marker(x)
        }
    }
}

fn parse(rule: &str) -> Vec<Alphabet> {
    rule.chars()
        .map(|c| Alphabet::new(c))
        .collect()
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

        info!("{:?}", state);

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
        let start = match self.start {
            Some(s) => parse(&s),
            None => parse("F")
        };
        let rules = match self.rules {
            Some(m) => m,
            None => Lrules::random()
        };
        let angle = match self.angle {
            Some(f) => f,
            None => 156./180.*PI
        };

        let rule_string = format!("{:?}", rules);
        let start_string = format!("{:?}", start);

        let description = format!(
            "generic L-System, n = {}, start = {}, rules = {}, angle = {}",
            iterations,
            start_string,
            rule_string,
            angle
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
