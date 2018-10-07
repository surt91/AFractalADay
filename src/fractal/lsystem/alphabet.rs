use fmt;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Serialize, Deserialize)]
pub enum Alphabet {
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
    pub fn new(c: char) -> Alphabet {
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

    pub fn parse(rule: &str) -> Vec<Alphabet> {
        rule.chars()
            .map(|c| Alphabet::new(c))
            .collect()
    }
}
