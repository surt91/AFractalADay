extern crate num;
use self::num::complex::Complex;

use fmt;

// adjust precision here
pub type Real = f32;
pub type Cplx = Complex<Real>;

pub type ComplexFunction = Box<Fn(Cplx) -> Cplx + Sync>;

#[derive(Debug)]
pub enum Coef {
    Real(Real),
    Complex(Cplx)
}

impl fmt::Display for Coef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Formula {
    pub callable: ComplexFunction,
    pub readable: String
}
