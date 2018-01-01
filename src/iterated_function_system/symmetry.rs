extern crate rand;
use self::rand::Rng;

use std::fmt;

#[derive(Debug, Clone)]
pub enum Symmetry {
    Rotational(usize),
    Vertical,
    Horizontal,
    None
}

impl Symmetry {
    pub fn random<T>(rng: &mut T) -> Symmetry
        where T: Rng
    {
        match rng.gen_range(0, 3) {
            1 => Symmetry::None,
            2 => Symmetry::Vertical,
            3 => Symmetry::Rotational(rng.gen_range(2, 8)),
            _ => unreachable!()
        }
    }

    pub fn name(&self) -> String {
        match *self {
            Symmetry::None => "no symmetry".to_owned(),
            Symmetry::Vertical => "mirror symmetry".to_owned(),
            Symmetry::Horizontal => "horizontal mirror symmetry".to_owned(),
            Symmetry::Rotational(x) => format!("{}-fold rotational symmetry", x),
        }
    }
}

impl fmt::Display for Symmetry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
