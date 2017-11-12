extern crate rand;
use self::rand::Rng;

use super::Variation;
use numbers::Real;
use std::f64::consts::PI as PI64;

const PI: Real = PI64 as Real;

#[derive(Debug, Clone)]
pub struct NonlinearTransformation {
    variation: Variation
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
        }
    }
}
