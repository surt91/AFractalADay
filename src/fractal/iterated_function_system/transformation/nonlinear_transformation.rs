use rand::{Rng, thread_rng};

use fractal::Variation;
use numbers::Real;
use std::f64::consts::PI as PI64;

const PI: Real = PI64 as Real;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonlinearTransformation {
    pub variation: Variation
}

impl NonlinearTransformation {
    pub fn new(variation: Variation) -> NonlinearTransformation {
        NonlinearTransformation {
            variation
        }
    }

    pub fn random<T>(rng: &mut T) -> NonlinearTransformation
        where T: Rng
    {
        let rn = rng.gen_range(0, Variation::num());
        let variation = Variation::from_number(rn).unwrap();

        NonlinearTransformation {
            variation
        }
    }

    pub fn name(&self) -> String {
        self.variation.name()
    }

    pub fn transform(&self, r: [Real; 2]) -> [Real; 2] {
        let x = r[0];
        let y = r[1];

        match self.variation {
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
