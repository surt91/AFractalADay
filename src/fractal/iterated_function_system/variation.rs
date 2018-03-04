use std::fmt;
use numbers::Real;
use rand::{Rng, thread_rng};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Variation {
    Linear,
    Sinusoidal,
    Spherical,
    Swirl,
    Horseshoe,
    Polar,
    Handkerchief,
    Heart,
    Disk,
    Spiral,
    Hyperbolic,
    Diamond,
    Ex,
    Julia,
    Bent,
    // Waves,
    Fisheye,
    // Popcorn,
    Exponential,
    Power,
    Cosine,
    // Rings,
    // Fan,
    Blob(Real, Real, Real),
    Pdj(Real, Real, Real, Real),
    Fan2(Real, Real),
}

impl Variation {
    pub fn num() -> usize {
        9
    }

    pub fn from_number(num: usize) -> Option<Variation> {
        match num {
            0 => Some(Variation::Linear),
            1 => Some(Variation::Sinusoidal),
            2 => Some(Variation::Spherical),
            3 => Some(Variation::Swirl),
            4 => Some(Variation::Horseshoe),
            5 => Some(Variation::Polar),
            6 => Some(Variation::Handkerchief),
            7 => Some(Variation::Heart),
            8 => Some(Variation::Disk),
            9 => Some(Variation::Spiral),
            10 => Some(Variation::Hyperbolic),
            11 => Some(Variation::Diamond),
            12 => Some(Variation::Ex),
            13 => Some(Variation::Julia),
            14 => Some(Variation::Bent),
            16 => Some(Variation::Fisheye),
            18 => Some(Variation::Exponential),
            19 => Some(Variation::Power),
            20 => Some(Variation::Cosine),
            23 => Some(Variation::Blob(
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
            )),
            24 => Some(Variation::Pdj(
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
            )),
            25 => Some(Variation::Fan2(
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
            )),
            _ => None
        }
    }

    pub fn from_string(name: &str) -> Option<Variation> {
        match name {
            "Linear" => Some(Variation::Linear),
            "Sinusoidal" => Some(Variation::Sinusoidal),
            "Spherical" => Some(Variation::Spherical),
            "Swirl" => Some(Variation::Swirl),
            "Horseshoe" => Some(Variation::Horseshoe),
            "Polar" => Some(Variation::Polar),
            "Handkerchief" => Some(Variation::Handkerchief),
            "Heart" => Some(Variation::Heart),
            "Disk" => Some(Variation::Disk),
            "Spiral" => Some(Variation::Spiral),
            "Hyperbolic" => Some(Variation::Hyperbolic),
            "Diamond" => Some(Variation::Diamond),
            "Ex" => Some(Variation::Ex),
            "Julia" => Some(Variation::Julia),
            "Bent" => Some(Variation::Bent),
            "Fisheye" => Some(Variation::Fisheye),
            "Exponential" => Some(Variation::Exponential),
            "Power" => Some(Variation::Power),
            "Cosine" => Some(Variation::Cosine),
            "Blob" => Some(Variation::Blob(
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
            )),
            "Pdj" => Some(Variation::Pdj(
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
            )),
            "Fan2" => Some(Variation::Fan2(
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
            )),
            s => s.parse::<usize>().ok().and_then(Variation::from_number)
        }
    }

    pub fn name(&self) -> String {
        match *self {
            Variation::Linear => "Linear",
            Variation::Sinusoidal => "Sinusoidal",
            Variation::Spherical => "Spherical",
            Variation::Swirl => "Swirl",
            Variation::Horseshoe => "Horseshoe",
            Variation::Polar => "Polar",
            Variation::Handkerchief => "Handkerchief",
            Variation::Heart => "Heart",
            Variation::Disk => "Disk",
            Variation::Spiral => "Spiral",
            Variation::Hyperbolic => "Hyperbolic",
            Variation::Diamond => "Diamond",
            Variation::Ex => "Ex",
            Variation::Julia => "Julia",
            Variation::Bent => "Bent",
            Variation::Fisheye => "Fisheye",
            Variation::Exponential => "Exponential",
            Variation::Power => "Power",
            Variation::Cosine => "Cosine",
            Variation::Blob(_, _, _) => "Blob",
            Variation::Pdj(_, _, _, _) => "Pdj",
            Variation::Fan2(_, _) => "Fan2",
        }.to_owned()
    }

    pub fn list() -> Vec<String> {
        (0..Variation::num()).map(|i| Variation::from_number(i)
                                                .unwrap()
                                                .name())
                             .collect()
    }
}

impl fmt::Display for Variation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
