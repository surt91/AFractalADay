use fmt;

use std::collections::HashMap;
use std::ops::Index;

use itertools::Itertools;

use super::Alphabet;


#[derive(Debug, Clone)]
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
    pub fn from_string(s: &str) -> Lrules {
        let mut rules: HashMap<Alphabet, Vec<Alphabet>> = HashMap::new();
        // defaults
        rules.insert(Alphabet::Invalid, Vec::new());
        rules.insert(Alphabet::F, vec![Alphabet::F]);
        rules.insert(Alphabet::M, vec![Alphabet::M]);
        rules.insert(Alphabet::P, vec![Alphabet::P]);
        rules.insert(Alphabet::BL, vec![Alphabet::BL]);
        rules.insert(Alphabet::BR, vec![Alphabet::BR]);

        for rule in s.split(',') {
            let mut it = rule.chars();
            let key = it.by_ref().skip_while(|&x| x == ' ').next().unwrap();

            // jump over :
            let delimiter = it.by_ref().skip_while(|&x| x == ' ').next().unwrap();
            if delimiter != ':' && delimiter != '→' {
                error!(
                    "Rule is not valid: not exactly one symbols as key in '{}', but: '{}'",
                    rule,
                    delimiter
                );
                panic!();
            }

            let rule = it.skip_while(|&x| x == ' ').collect::<String>();
            rules.insert(Alphabet::new(key), Alphabet::parse(&rule));
        }

        Lrules{
            rules
        }
    }

    pub fn random() -> Lrules {
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
