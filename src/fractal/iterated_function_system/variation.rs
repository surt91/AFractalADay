use std::fmt;
use rand::{Rng, thread_rng};

use numbers::Real;
use std::f64::consts::PI as PI64;

const PI: Real = PI64 as Real;

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

    pub fn apply(&self, r: [Real; 2]) -> [Real; 2] {
        let x = r[0];
        let y = r[1];

        match *self {
            Variation::Linear => r,
            Variation::Sinusoidal => [x.sin(), y.sin()],
            Variation::Spherical => {
                let r2 = x*x + y*y;
                [x/r2, y/r2]
            },
            Variation::Swirl => {
                let r2 = x*x + y*y;
                [x*r2.sin() - y*r2.cos(), x*r2.cos() + y*r2.sin()]
            },
            Variation::Horseshoe => {
                let r = (x*x + y*y).sqrt();
                [(x-y)*(x+y) / r, 2.*x*y / r]
            },
            Variation::Polar => {
                let r = (x*x + y*y).sqrt();
                let theta = (x/y).atan();
                [theta / PI, r - 1.]
            },
            Variation::Handkerchief => {
                let r = (x*x + y*y).sqrt();
                let theta = (x/y).atan();
                [r*(theta+r).sin(), (theta-r).cos()]
            }
            Variation::Heart => {
                let r = (x*x + y*y).sqrt();
                let theta = (x/y).atan();
                [r*(theta*r).sin(), -(theta*r).cos()]
            }
            Variation::Disk => {
                let r = (x*x + y*y).sqrt();
                let theta = (x/y).atan();
                [theta/PI * (r*PI).sin(), theta/PI * (r*PI).cos()]
            }
            Variation::Spiral => {
                let r = (x*x + y*y).sqrt();
                let theta = (x/y).atan();
                [(theta.cos() + r.sin()) / r, (theta.sin() - r.cos()) / r]
            }
            Variation::Hyperbolic => {
                let r = (x*x + y*y).sqrt();
                let theta = (x/y).atan();
                [theta.sin() / r, r * theta.cos()]
            }
            Variation::Diamond => {
                let r = (x*x + y*y).sqrt();
                let theta = (x/y).atan();
                [theta.sin() * r.cos(), theta.cos() * r.sin()]
            }
            Variation::Ex => {
                let r = (x*x + y*y).sqrt();
                let theta = (x/y).atan();
                let p0 = (theta + r).sin();
                let p1 = (theta - r).cos();
                let p03 = p0.powi(3);
                let p13 = p1.powi(3);
                [r * (p03 + p13), r * (p03 - p13)]
            }
            Variation::Julia => {
                let r = (x*x + y*y).sqrt();
                let theta = (x/y).atan();
                let sqrt_r = r.sqrt();
                let omega = thread_rng().gen_range(0, 1) as Real * PI;
                let arg = theta/2. + omega;
                [sqrt_r * arg.cos(), sqrt_r * arg.sin()]
            }
            Variation::Bent => {
                if x >= 0. && y >= 0. {
                    [x, y]
                } else if x < 0. && y >= 0. {
                    [2.*x, y]
                } else if x >= 0. && y < 0. {
                    [x, y/2.]
                } else {
                    [2.*x, y/2.]
                }
            }
            Variation::Fisheye => {
                let r = (x*x + y*y).sqrt();
                let ir = 2. / (r + 1.);
                [ir * y, ir * x]
            }
            Variation::Exponential => {
                let f = (x - 1.).exp();
                [f * (PI * y).cos(), f * (PI * y).sin()]
            }
            Variation::Power => {
                let r = (x*x + y*y).sqrt();
                let theta = (x/y).atan();
                let f = r.powf(theta.sin());
                [f * theta.cos(), f * theta.sin()]
            }
            Variation::Cosine => {
                [(PI*x).cos() * y.cosh(), -(PI*x).sin() * y.sinh()]
            }
            Variation::Blob(p1, p2, p3) => {
                let r = (x*x + y*y).sqrt();
                let theta = (x/y).atan();
                let f = r * (p2 + (p1 - p2)/2. * ((p3*theta).sin() + 1.));
                [f * theta.cos(), f * theta.sin()]
            }
            Variation::Pdj(p1, p2, p3, p4) => {
                [(p1*y).sin() - (p2*x).cos(), (p3*x).sin() - (p4*y).cos()]
            }
            Variation::Fan2(p1, p2) => {
                let r = (x*x + y*y).sqrt();
                let theta = (x/y).atan();
                let t = theta + p2 - p1 * (2.*theta*p2/p1).floor();
                if t > p1/ 2. {
                    [r * (theta - p1/2.).sin(), r * (theta - p1/2.).cos()]
                } else {
                    [r * (theta + p1/2.).sin(), r * (theta + p1/2.).cos()]
                }
            }
        }
    }
}

impl fmt::Display for Variation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
