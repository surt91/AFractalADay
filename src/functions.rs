extern crate rand;
use self::rand::Rng;

extern crate num;
use self::num::complex::Complex;

use std::fmt::Display;

pub enum Coef {
    Real(f64),
    Complex(Complex<f64>)
}

pub struct Terms {
    candidates_real: Vec<Box<Fn(f64) -> (Box<Fn(Complex<f64>) -> Complex<f64> + Sync>, String)>>,
    candidates_comp: Vec<Box<Fn(Complex<f64>) -> (Box<Fn(Complex<f64>) -> Complex<f64> + Sync>, String)>>,
}

impl Terms {
    pub fn new() -> Terms{
        Terms {candidates_real: Terms::generate_candidates(),
               candidates_comp: Terms::generate_candidates()}
    }

    fn generate_candidates<T: 'static + Display + Sync + Copy + Into<Complex<f64>>>() -> Vec<Box<Fn(T) -> (Box<Fn(Complex<f64>) -> Complex<f64> + Sync>, String)>> {
        let mut candidates: Vec<Box<Fn(T) -> (Box<Fn(Complex<f64>) -> Complex<f64> + Sync>, String)>> = Vec::new();

        candidates.push(Box::new(|a| (Box::new(move |_: Complex<f64>| a.into() ),
                                           format!("{}", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a.into() * x),
                                           format!("{} z", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a.into() * x.powf(2.)),
                                           format!("{} z²", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a.into() * x.powf(3.)),
                                           format!("{} z³", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a.into() * x.powf(4.)),
                                           format!("{} z⁴", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a.into() * x.powf(5.)),
                                           format!("{} z⁵", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a.into() * x.powf(6.)),
                                           format!("{} z⁶", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a.into() * x.powf(7.)),
                                           format!("{} z⁷", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a.into() * x.sin()),
                                           format!("{} sin(z)", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a.into() * x.cos()),
                                           format!("{} cos(z)", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a.into() * x.tan()),
                                           format!("{} tan(z)", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a.into() * x.sinh()),
                                           format!("{} sinh(z)", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a.into() * x.cosh()),
                                           format!("{} cosh(z)", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a.into() * x.atanh()),
                                           format!("{} artanh(z)", a))));
        for b in 2..8i8 {
            candidates.push(Box::new(move |a| (Box::new(move |x: Complex<f64>| a.into() * (x*(b as f64).ln()).exp() ),
                                               format!("{} {}ᶻ", a, b))));
        }
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a.into() * x.exp() ),
                                           format!("{} exp(z)", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a.into() * x.ln() ),
                                           format!("{} ln(z)", a))));
        candidates
    }

    pub fn choice(&mut self, a: Coef, rng: &mut rand::StdRng) -> (Box<Fn(Complex<f64>) -> Complex<f64> + Sync>, String) {
        let num = self.candidates_real.len();
        let idx = rng.gen_range(0, num as usize);
        match a {
            Coef::Real(x) => {self.candidates_comp.swap_remove(idx); self.candidates_real.swap_remove(idx)(x)},
            Coef::Complex(z) => {self.candidates_real.swap_remove(idx); self.candidates_comp.swap_remove(idx)(z)},
        }
    }
}
