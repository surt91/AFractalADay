extern crate rand;
use self::rand::Rng;

extern crate num;
use self::num::complex::Complex;

use std::fmt::Display;

pub enum Coef {
    Real(f64),
    Complex(Complex<f64>)
}

pub struct Formula {
    pub callable: Box<Fn(Complex<f64>) -> Complex<f64> + Sync>,
    pub readable: String
}

pub struct Terms {
    pub candidates_real: Vec<Box<Fn(f64) -> Formula>>,
    pub candidates_comp: Vec<Box<Fn(Complex<f64>) -> Formula>>,
}

impl Terms {
    pub fn new() -> Terms{
        Terms {candidates_real: Terms::generate_candidates(),
               candidates_comp: Terms::generate_candidates()}
    }

    fn generate_candidates<T: 'static + Display + Sync + Copy + Into<Complex<f64>>>() -> Vec<Box<Fn(T) -> Formula>> {
        let mut candidates: Vec<Box<Fn(T) -> Formula>> = Vec::new();

        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |_: Complex<f64>| a.into() ),
                                        readable: format!("({})", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| a.into() * x),
                                        readable: format!("({}) z", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| a.into() * x.powf(2.)),
                                        readable: format!("({}) z²", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| a.into() * x.powf(3.)),
                                        readable: format!("({}) z³", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| a.into() * x.powf(4.)),
                                        readable: format!("({}) z⁴", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| a.into() * x.powf(5.)),
                                        readable: format!("({}) z⁵", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| a.into() * x.powf(6.)),
                                        readable: format!("({}) z⁶", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| a.into() * x.powf(7.)),
                                        readable: format!("({}) z⁷", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| a.into() * x.sin()),
                                        readable: format!("({}) sin(z)", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| a.into() * x.cos()),
                                        readable: format!("({}) cos(z)", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| a.into() * x.tan()),
                                        readable: format!("({}) tan(z)", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| a.into() * x.sinh()),
                                        readable: format!("({}) sinh(z)", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| a.into() * x.cosh()),
                                        readable: format!("({}) cosh(z)", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| a.into() * x.atanh()),
                                        readable: format!("({}) artanh(z)", a)
                                    }));
        for b in 2..8i8 {
            candidates.push(Box::new(move |a| Formula {
                                            callable: Box::new(move |x: Complex<f64>| a.into() * (x*(b as f64).ln()).exp() ),
                                            readable:     format!("({}) {}ᶻ", a, b)
                                        }));
        }
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| (x*a.into().ln()).exp() ),
                                        readable: format!("({})ᶻ", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| a.into() * x.exp() ),
                                        readable: format!("({}) exp(z)", a)
                                    }));
        candidates.push(Box::new(|a| Formula {
                                        callable: Box::new(move |x: Complex<f64>| a.into() * x.ln() ),
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
