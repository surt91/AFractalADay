extern crate rand;
use self::rand::Rng;

use numbers::Real;

#[derive(Debug, Clone)]
pub struct AffineTransformation {
    parameters: [Real; 6],
}

impl AffineTransformation {
    pub fn new(a: Real, b: Real, c: Real, d: Real, e: Real, f: Real) -> AffineTransformation {
        AffineTransformation {
            parameters: [a, b, c, d, e, f]
        }
    }

    pub fn random(rng: &mut rand::StdRng) -> AffineTransformation {
        // The parameters should be contracting (in the average)
        // that means: a^2+c^2 < 1, b^2+d^2 < 1
        // also values near zero will lead to thin lines in the fractal
        // therefore, we scale the random numbers and set an offset
        AffineTransformation {
            parameters: [rng.gen::<f32>() * 0.7 + 0.1,
                         rng.gen::<f32>() * 0.7 + 0.1,
                         rng.gen::<f32>() * 0.7 + 0.1,
                         rng.gen::<f32>() * 0.7 + 0.1,
                         rng.gen::<f32>() * 0.7 + 0.1,
                         rng.gen::<f32>() * 0.7 + 0.1]
        }
    }

    pub fn transform(&self, x: [Real; 2]) -> [Real; 2] {
        let mut out = [0.; 2];
        out[0] = self.parameters[0] * x[0] + self.parameters[1] * x[1] + self.parameters[2];
        out[1] = self.parameters[3] * x[0] + self.parameters[4] * x[1] + self.parameters[5];
        out
    }
}