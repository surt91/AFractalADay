use rand::Rng;

use serde::{self, Serialize, Deserialize};

use crate::numbers::{Real,Cplx};

mod affine_transformation;
mod mobius_transformation;
mod nonlinear_transformation;

pub use self::affine_transformation::AffineTransformation;
pub use self::mobius_transformation::MobiusTransformation;
pub use self::nonlinear_transformation::NonlinearTransformation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Transformation {
    Affine(AffineTransformation),
    Mobius(MobiusTransformation)
}

impl Transformation {
    pub fn affine(a: Real, b: Real, c: Real, d: Real, e: Real, f: Real) -> Transformation {
        Transformation::Affine(AffineTransformation::new(a, b, c, d, e, f))
    }
    pub fn mobius(a: Cplx, b: Cplx, c: Cplx, d: Cplx) -> Transformation {
        Transformation::Mobius(MobiusTransformation::new(a, b, c, d))
    }

    pub fn random<T>(rng: &mut T) -> Transformation
        where T: Rng
    {
        if rng.gen::<Real>() > 0.5 {
            Transformation::Mobius(MobiusTransformation::random(rng))
        } else {
            Transformation::Affine(AffineTransformation::random(rng))
        }
    }

    pub fn identity() -> Transformation {
        Transformation::Affine(AffineTransformation::identity())
    }

    pub fn transform(&self, p: [Real; 2]) -> [Real; 2]
    {
        match *self {
            Transformation::Affine(ref x) => {
                x.transform(p)
            },
            Transformation::Mobius(ref x) => {
                x.transform(p)
            }
        }
    }
}
