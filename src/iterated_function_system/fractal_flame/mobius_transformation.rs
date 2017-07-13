extern crate rand;
use self::rand::Rng;

use numbers::{Real, Cplx};
use std::ops::Mul;

/// A Mobius transformation has 4 complex parameters and maps a complex number z
/// T(z) = (a*z + b) / (c*z + d)
#[derive(Debug, Clone)]
pub struct MobiusTransformation {
    parameters: [Cplx; 4],
}

impl MobiusTransformation {
    pub fn new(a: Cplx, b: Cplx, c: Cplx, d: Cplx) -> MobiusTransformation {
        MobiusTransformation {
            parameters: [a, b, c, d]
        }
    }

    pub fn random(rng: &mut rand::StdRng) -> MobiusTransformation {
        MobiusTransformation {
            parameters: [
                Cplx::new(rng.gen::<Real>() * 2.4 - 1.2, rng.gen::<Real>() * 2.4 - 1.2,),
                Cplx::new(rng.gen::<Real>() * 2.4 - 1.2, rng.gen::<Real>() * 2.4 - 1.2,),
                Cplx::new(rng.gen::<Real>() * 2.4 - 1.2, rng.gen::<Real>() * 2.4 - 1.2,),
                Cplx::new(rng.gen::<Real>() * 2.4 - 1.2, rng.gen::<Real>() * 2.4 - 1.2,)
            ]
        }
    }

    pub fn transform(&self, z: Cplx) -> Cplx {
        (self.parameters[0]*z + self.parameters[1]) / (self.parameters[2]*z + self.parameters[3])
    }
}

impl Mul<Cplx> for MobiusTransformation {
    type Output = Self;
    fn mul(self, rhs: Cplx) -> Self {
        let p = self.parameters;
        MobiusTransformation::new(rhs*p[0], rhs*p[1], p[2], p[3])
    }
}
