use std::fmt;

#[derive(Debug, Clone)]
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
