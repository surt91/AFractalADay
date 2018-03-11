use rand::Rng;
use rand::distributions::{Normal, IndependentSample};

use numbers::{Real, Cplx};

/// A Mobius transformation has 4 complex parameters and maps a complex number z
/// T(z) = (a*z + b) / (c*z + d)
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        let normal = Normal::new(0., 0.6);
        MobiusTransformation {
            parameters: [
                normal.ind_sample(rng) as Real, normal.ind_sample(rng) as Real,
                normal.ind_sample(rng) as Real, normal.ind_sample(rng) as Real,
                normal.ind_sample(rng) as Real, normal.ind_sample(rng) as Real,
                normal.ind_sample(rng) as Real, normal.ind_sample(rng) as Real,
            ]
        }
    }

    pub fn transform(&self, p: [Real; 2]) -> [Real; 2] {
        let z = Cplx::new(p[0], p[1]);
        let a = Cplx::new(self.parameters[0], self.parameters[1]);
        let b = Cplx::new(self.parameters[2], self.parameters[3]);
        let c = Cplx::new(self.parameters[4], self.parameters[5]);
        let d = Cplx::new(self.parameters[6], self.parameters[7]);

        let tmp = (a*z + b) / (c*z + d);
        [tmp.re, tmp.im]
    }
}
