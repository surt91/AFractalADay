use std::fmt;
use numbers::Real;
use rand::{Rng, thread_rng};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Rand)]
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
    pub fn from_number(num: usize) -> Option<Variation> {
        Variation::from_string(&format!("{}", num))
    }

    pub fn from_string(name: &str) -> Option<Variation> {
        match name {
            "Linear" | "0" => Some(Variation::Linear),
            "Sinusoidal" | "1" => Some(Variation::Sinusoidal),
            "Spherical" | "2" => Some(Variation::Spherical),
            "Swirl" | "3" => Some(Variation::Swirl),
            "Horseshoe" | "4" => Some(Variation::Horseshoe),
            "Polar" | "5" => Some(Variation::Polar),
            "Handkerchief" | "6" => Some(Variation::Handkerchief),
            "Heart" | "7" => Some(Variation::Heart),
            "Disk" | "8" => Some(Variation::Disk),
            "Spiral" | "9" => Some(Variation::Spiral),
            "Hyperbolic" | "10" => Some(Variation::Hyperbolic),
            "Diamond" | "11" => Some(Variation::Diamond),
            "Ex" | "12" => Some(Variation::Ex),
            "Julia" | "13" => Some(Variation::Julia),
            "Bent" | "14" => Some(Variation::Bent),
            "Fisheye" | "16" => Some(Variation::Fisheye),
            "Exponential" | "18" => Some(Variation::Exponential),
            "Power" | "19" => Some(Variation::Power),
            "Cosine" | "20" => Some(Variation::Cosine),
            "Blob" | "23" => Some(Variation::Blob(
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
            )),
            "Pdj" | "24" => Some(Variation::Pdj(
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
                thread_rng().gen_range(0.,1.),
            )),
            "Fan2" | "25" => Some(Variation::Fan2(
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
        let implemented = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 18, 19, 20, 23, 24, 25];
        implemented.iter()
                   .map(|&i| Variation::from_number(i)
                                      .map_or("n/a".to_owned(), |x| x.name()))
                   .collect()
    }
}

impl fmt::Display for Variation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
