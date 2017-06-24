extern crate rand;
use self::rand::Rng;

use numbers::Real;

#[derive(Debug, Clone)]
pub enum Variation {
    Linear,
    Sinusoidal,
    Spherical,
    Swirl,
    Horseshoe,
}

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

    pub fn random(rng: &mut rand::StdRng) -> NonlinearTransformation {
        let variation = match rng.gen_range(0, 5) {
            0 => Variation::Linear,
            1 => Variation::Sinusoidal,
            2 => Variation::Spherical,
            3 => Variation::Swirl,
            4 => Variation::Horseshoe,
            _ => unreachable!()
        };

        NonlinearTransformation {
            variation
        }
    }

    pub fn name(&self) -> String {
        match self.variation {
            Variation::Linear => "Linear",
            Variation::Sinusoidal => "Sinusoidal",
            Variation::Spherical => "Spherical",
            Variation::Swirl => "Swirl",
            Variation::Horseshoe => "Horseshoe"
        }.to_owned()
    }

    pub fn transform(&self, r: [Real; 2]) -> [Real; 2] {
        let x = r[0];
        let y = r[1];

        match self.variation {
            Variation::Linear => r,
            Variation::Sinusoidal => [x.sin(), y.sin()],
            Variation::Spherical => {let r2 = x*x + y*y; [x/r2, y/r2]},
            Variation::Swirl => {let r2 = x*x + y*y; [x*r2.sin() - y*r2.cos(), x*r2.cos() + y*r2.sin()]},
            Variation::Horseshoe => {let r = (x*x + y*y).sqrt(); [(x-y)*(x+y) / r, 2.*x*y / r]}
        }
    }
}
