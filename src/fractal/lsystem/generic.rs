use std::f64::consts::PI;

use fractal::FractalBuilder;

use std::collections::HashMap;
use std::ops::Index;
use fmt;
use itertools::Itertools;

use super::LSystem;
use super::turtle::{Turtle, Canvas};

extern crate rayon;
use self::rayon::prelude::*;

#[derive(Debug)]
pub struct Lrules {
    rules: HashMap<Alphabet, Vec<Alphabet>>
}

impl Lrules {
    /// each rule consists of a key and a value, such that every key in the state will be replaced
    /// by the value, they are separated by a colon ':'
    /// the single rules are speparated by commas ','
    /// e.g. R:+RF-LFL-FR+,L:-LF+RFR+FL-
    /// the rule for F is by default the identity F:F
    /// only upper case letters are valid symbols
    fn from_string(s: &str) -> Lrules {
        let mut rules: HashMap<Alphabet, Vec<Alphabet>> = HashMap::new();
        // defaults
        rules.insert(Alphabet::Invalid, Vec::new());
        rules.insert(Alphabet::F, vec![Alphabet::F]);
        rules.insert(Alphabet::M, vec![Alphabet::M]);
        rules.insert(Alphabet::P, vec![Alphabet::P]);
        rules.insert(Alphabet::BL, vec![Alphabet::BL]);
        rules.insert(Alphabet::BR, vec![Alphabet::BR]);

        for rule in s.split(',') {
            rule.trim();
            let mut it = rule.chars();
            let key = it.next().unwrap();
            // jump over :
            if it.next().unwrap() != ':' {
                error!("Rule is not valid: not exactly one symbols as key in '{}'", rule);
                panic!();
            }
            let rule = it.collect::<String>();
            rules.insert(Alphabet::new(key), parse(&rule));
        }

        Lrules{
            rules
        }
    }

    fn random() -> Lrules {
        Lrules::from_string("F:--FFF+")
    }
}

impl fmt::Display for Lrules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rule_strings: Vec<String> = Vec::new();
        for (key, value) in self.rules.iter() {
            let rule_string = value.iter()
                .cloned()
                .map(|a| format!("{}", a))
                .join("");

            match key {
                Alphabet::F | Alphabet::Marker(_)
                    => rule_strings.push(format!("{}:{}", key, rule_string)),
                _ => ()
            };
        }
        write!(f, "{}", rule_strings.join(", "))
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

impl fmt::Display for Alphabet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = match &self {
            Alphabet::F => 'F',
            Alphabet::M => '-',
            Alphabet::P => '+',
            Alphabet::BL => '[',
            Alphabet::BR => ']',
            Alphabet::Marker(c) => *c,
            Alphabet::Invalid => '#'
        };

        write!(f, "{}", out)
    }
}

impl Alphabet {
    fn new(c: char) -> Alphabet {
        match c {
            'F' => Alphabet::F,
            '[' => Alphabet::BL,
            ']' => Alphabet::BR,
            '+' => Alphabet::P,
            '-' => Alphabet::M,
            'A' ... 'Z'  => Alphabet::Marker(c),
            _ => Alphabet::Invalid
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
