use rand::Rng;

use crate::numbers::Real;
use std::ops::Mul;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AffineTransformation {
    parameters: [Real; 6],
    pub symmetry: bool,
}

impl AffineTransformation {
    pub fn new(a: Real, b: Real, c: Real, d: Real, e: Real, f: Real) -> AffineTransformation {
        AffineTransformation {
            parameters: [a, b, c, d, e, f],
            symmetry: false
        }
    }

    pub fn random<T>(rng: &mut T) -> AffineTransformation
        where T: Rng
    {
        // The parameters should be contracting (in the average)
        // that means: a^2+c^2 < 1, b^2+d^2 < 1
        AffineTransformation {
            parameters: [rng.gen::<Real>() * 2.4 - 1.2,
                         rng.gen::<Real>() * 2.4 - 1.2,
                         rng.gen::<Real>() * 2.4 - 1.2,
                         rng.gen::<Real>() * 2.4 - 1.2,
                         rng.gen::<Real>() * 2.4 - 1.2,
                         rng.gen::<Real>() * 2.4 - 1.2],
            symmetry: false
        }
    }

    pub fn transform(&self, x: [Real; 2]) -> [Real; 2] {
        let mut out = [0.; 2];
        out[0] = self.parameters[0] * x[0] + self.parameters[1] * x[1] + self.parameters[2];
        out[1] = self.parameters[3] * x[0] + self.parameters[4] * x[1] + self.parameters[5];
        out
    }

    pub fn identity() -> AffineTransformation {
        AffineTransformation {
            parameters: [1., 0., 0.,
                         0., 1., 0.],
            symmetry: true
        }
    }

    pub fn vertical_mirror() -> AffineTransformation {
        AffineTransformation {
            parameters: [-1., 0., 0.,
                          0., 1., 0.],
            symmetry: true
        }
    }

    pub fn horizontal_mirror() -> AffineTransformation {
        AffineTransformation {
            parameters: [1.,  0., 0.,
                         0., -1., 0.],
            symmetry: true
        }
    }

    pub fn rotate(phi: Real) -> AffineTransformation {
        AffineTransformation {
            parameters: [phi.cos(), -phi.sin(), 0.,
                         phi.sin(),  phi.cos(), 0.],
            symmetry: true
        }
    }
}

impl Mul<Real> for AffineTransformation {
    type Output = Self;
    fn mul(self, rhs: Real) -> Self {
        let p = self.parameters;
        AffineTransformation::new(rhs*p[0], rhs*p[1], rhs*p[2], rhs*p[3], rhs*p[4], rhs*p[5])
    }
}
