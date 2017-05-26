extern crate rand;
use self::rand::Rng;

extern crate num;
use self::num::complex::Complex;

use std::fmt::Display;

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

#[derive(Default)]
pub struct Terms {
    pub candidates_real: Vec<Box<Fn(Real) -> Formula>>,
    pub candidates_comp: Vec<Box<Fn(Cplx) -> Formula>>,
}

impl Terms {
    pub fn new() -> Terms{
        Terms {candidates_real: Terms::generate_candidates(),
               candidates_comp: Terms::generate_candidates()}
    }

    fn generate_candidates<T: 'static + Display + Sync + Copy + Into<Cplx>>() -> Vec<Box<Fn(T) -> Formula>> {
        let mut candidates: Vec<Box<Fn(T) -> Formula>> = Vec::new();

        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |_| a.into() ),
                                        readable: format!("({})", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| a.into() * x),
                                        readable: format!("({}) z", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| a.into() * x.powf(2.)),
                                        readable: format!("({}) z²", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| a.into() * x.powf(3.)),
                                        readable: format!("({}) z³", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| a.into() * x.powf(4.)),
                                        readable: format!("({}) z⁴", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| a.into() * x.powf(5.)),
                                        readable: format!("({}) z⁵", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| a.into() * x.powf(6.)),
                                        readable: format!("({}) z⁶", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| a.into() * x.powf(7.)),
                                        readable: format!("({}) z⁷", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| a.into() * x.sin()),
                                        readable: format!("({}) sin(z)", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| a.into() * x.cos()),
                                        readable: format!("({}) cos(z)", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| a.into() * x.tan()),
                                        readable: format!("({}) tan(z)", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| a.into() * x.sinh()),
                                        readable: format!("({}) sinh(z)", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| a.into() * x.cosh()),
                                        readable: format!("({}) cosh(z)", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| a.into() * x.atanh()),
                                        readable: format!("({}) artanh(z)", a)
                                    }));
        for b in 2..8i8 {
            candidates.push(Box::new(move |a| Formula {
                                            callable: Box::new(move |x| a.into() * (x*(b as Real).ln()).exp() ),
                                            readable:     format!("({}) {}ᶻ", a, b)
                                        }));
        }
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| (x*a.into().ln()).exp() ),
                                        readable: format!("({})ᶻ", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| a.into() * x.exp() ),
                                        readable: format!("({}) exp(z)", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x| a.into() * x.ln() ),
                                        readable: format!("({}) ln(z)", a)
                                    }));
        candidates
    }

    pub fn choice(&mut self, a: Coef, rng: &mut rand::StdRng) -> Formula {
        let num = self.candidates_real.len();
        let idx = rng.gen_range(0, num as usize);
        match a {
            Coef::Real(x) => {self.candidates_comp.swap_remove(idx); self.candidates_real.swap_remove(idx)(x)},
            Coef::Complex(z) => {self.candidates_real.swap_remove(idx); self.candidates_comp.swap_remove(idx)(z)},
        }
    }
}
