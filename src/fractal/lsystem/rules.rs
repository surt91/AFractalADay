use crate::fmt;

use std::collections::HashMap;
use std::ops::Index;
use std::iter;

use rand::{Rng, FromEntropy, SeedableRng};
use rand::seq::SliceRandom;

use itertools::Itertools;

use super::Alphabet;
use crate::fractal::{RngType};

use serde::{self, Serialize, Deserialize};
use serde::ser::Serializer;
use serde::Deserializer;
use log::error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lrules {
    start: Vec<Alphabet>,
    rules: HashMap<Alphabet, Vec<Alphabet>>,
}

pub fn lrule_serialize<S>(x: &Lrules, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&format!("{}&{}", x.start.iter().map(|a| a.to_string()).join(""), x))
}

pub fn lrule_deserialize<'de, D>(deserializer: D) -> Result<Lrules, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let mut split = s.split("&");
    let start_string = split.next().unwrap();
    let rule_string = split.next().unwrap();
    Ok(Lrules::from_string(start_string, &rule_string))
}

impl Lrules {
    /// each rule consists of a key and a value, such that every key in the state will be replaced
    /// by the value, they are separated by a colon ':'
    /// the single rules are speparated by commas ','
    /// e.g. R:+RF-LFL-FR+,L:-LF+RFR+FL-
    /// the rule for F is by default the identity F:F
    /// only upper case letters are valid symbols
    pub fn from_string(start_string: &str, rule_string: &str) -> Lrules {
        let start = start_string.chars()
                                .map(|c| Alphabet::new(c))
                                .collect();

        let mut rules: HashMap<Alphabet, Vec<Alphabet>> = HashMap::new();
        // defaults
        rules.insert(Alphabet::Invalid, Vec::new());
        rules.insert(Alphabet::F, vec![Alphabet::F]);
        rules.insert(Alphabet::M, vec![Alphabet::M]);
        rules.insert(Alphabet::P, vec![Alphabet::P]);
        rules.insert(Alphabet::BL, vec![Alphabet::BL]);
        rules.insert(Alphabet::BR, vec![Alphabet::BR]);

        for rule in rule_string.split(',') {
            let mut it = rule.chars();
            let key = it.by_ref().skip_while(|&x| x == ' ' || x == '\n').next().unwrap();

            // jump over :
            let delimiter = it.by_ref().skip_while(|&x| x == ' ' || x == '\n').next().unwrap();
            if delimiter != ':' && delimiter != '→' {
                error!(
                    "Rule is not valid: not exactly one symbols as key in '{}', but: '{}'",
                    rule,
                    delimiter
                );
                panic!();
            }

            let rule = it.skip_while(|&x| x == ' ' || x == '\n').collect::<String>();
            rules.insert(Alphabet::new(key), Alphabet::parse(&rule));
        }

        Lrules{
            start,
            rules,
        }
    }

    pub fn random(seed: Option<u64>) -> Lrules {
        // first select a random number of symbols, the symbolset
        let mut rng = match seed {
            Some(s) => RngType::seed_from_u64(s),
            None => RngType::from_entropy()
        };
        let num_symbols = rng.gen_range(0, 5);
        let symbolset: Vec<Alphabet> = iter::repeat(())
            .map(|()| {
                Alphabet::new(
                    *"ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars()
                        .collect::<Vec<char>>()
                        .choose(&mut rng)
                        .unwrap()
                )
            })
            .take(num_symbols)
            .chain(iter::once(Alphabet::new('F')))
            .collect();

        // select a random start from symbols of the symbolset
        let num_start = rng.gen_range(0, 5);
        let start = iter::repeat(())
                         .map(|()| symbolset.choose(&mut rng).unwrap().clone())
                         .take(num_start)
                         .collect::<Vec<Alphabet>>();

        // generate a random rule for each elemtent of the symbolset
        let mut rules: HashMap<Alphabet, Vec<Alphabet>> = HashMap::new();
        let mut allowed_symbols = symbolset.clone();
        allowed_symbols.push(Alphabet::new(']'));
        allowed_symbols.push(Alphabet::new('+'));
        allowed_symbols.push(Alphabet::new('-'));
        rules.insert(Alphabet::new('['), vec![Alphabet::new('[')]);
        rules.insert(Alphabet::new(']'), vec![Alphabet::new(']')]);
        rules.insert(Alphabet::new('+'), vec![Alphabet::new('+')]);
        rules.insert(Alphabet::new('-'), vec![Alphabet::new('-')]);

        for s in &symbolset {
            let num = rng.gen_range(1, 10);
            let tmp = iter::repeat(())
                             .map(|()| allowed_symbols.choose(&mut rng).unwrap().clone())
                             .take(num)
                             .collect::<Vec<Alphabet>>();

            // search for closing brackets and add a opening bracket
            let mut rule: Vec<Alphabet> = Vec::new();
            for (n, c) in tmp.iter().enumerate() {
                if c == &Alphabet::new(']') {
                    let idx = rng.gen_range(0, n+1);
                    rule.insert(idx, Alphabet::new('['));
                }
                rule.push(c.clone());
            }

            rules.insert(s.clone(), rule);
        }

        Lrules {
            start,
            rules,
        }
    }

    pub fn start<'a>(&'a self) -> &'a Vec<Alphabet> {
        &self.start
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
                    => rule_strings.push(format!("{} → {}", key, rule_string)),
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
