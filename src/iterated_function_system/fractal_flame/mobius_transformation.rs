extern crate rand;
use self::rand::Rng;

use numbers::{Real, Cplx};

/// A Mobius transformation has 4 complex parameters and maps a complex number z
/// T(z) = (a*z + b) / (c*z + d)
#[derive(Debug, Clone)]
pub struct MobiusTransformation {
    parameters: [Real; 8],
}

impl MobiusTransformation {
    pub fn new(a: Cplx, b: Cplx, c: Cplx, d: Cplx) -> MobiusTransformation {
        MobiusTransformation {
            parameters: [
                a.re, a.im,
                b.re, b.im,
                c.re, c.im,
                d.re, d.im
            ]
        }
    }

    pub fn random<T>(rng: &mut T) -> MobiusTransformation
        where T: Rng
    {
        MobiusTransformation {
            parameters: [
                rng.gen::<Real>() * 2.4 - 1.2, rng.gen::<Real>() * 2.4 - 1.2,
                rng.gen::<Real>() * 2.4 - 1.2, rng.gen::<Real>() * 2.4 - 1.2,
                rng.gen::<Real>() * 2.4 - 1.2, rng.gen::<Real>() * 2.4 - 1.2,
                rng.gen::<Real>() * 2.4 - 1.2, rng.gen::<Real>() * 2.4 - 1.2
            ]
        }
    }

    pub fn transform(&self, x: Real, y: Real) -> [Real; 2] {
        let z = Cplx::new(x, y);
        let a = Cplx::new(self.parameters[0], self.parameters[1]);
        let b = Cplx::new(self.parameters[2], self.parameters[3]);
        let c = Cplx::new(self.parameters[4], self.parameters[5]);
        let d = Cplx::new(self.parameters[6], self.parameters[7]);

        let tmp = (a*z + b) / (c*z + d);
        [tmp.re, tmp.im]
    }
}
