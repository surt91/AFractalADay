extern crate num;
use self::num::complex::Complex;

// adjust precision here
pub type Real = f32;
pub type Cplx = Complex<Real>;

pub type ComplexFunction = Box<Fn(Cplx) -> Cplx + Sync>;

pub enum Coef {
    Real(Real),
    Complex(Cplx)
}

pub struct Formula {
    pub callable: ComplexFunction,
    pub readable: String
}
