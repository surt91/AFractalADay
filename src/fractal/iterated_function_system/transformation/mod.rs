use numbers::{Real,Cplx};

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
}
