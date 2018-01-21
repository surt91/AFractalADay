use rand::Rng;

use std::fmt::Display;

use numbers::{Cplx, Real, Coef, ComplexFunction, Formula};

pub fn random_coef<T: Rng>(rng: &mut T) -> Coef {
    let a_re = ((rng.gen_range(1f64, 2.) * 10.).floor() / 10.) as Real;
    let a_im = ((rng.gen_range(1f64, 2.) * 10.).floor() / 10.) as Real;
    if rng.gen::<f64>() < 0.1 {
        let tmp = Cplx::new(a_re, a_im);
        Coef::Complex(tmp)
    } else if rng.gen::<f64>() < 0.4 {
        let tmp = a_re;
        Coef::Real(tmp)
    } else {
        Coef::Real(1.)
    }
}

/// Calculates the derivative of f at z.
///
/// `https://en.wikipedia.org/wiki/Numerical_differentiation#Complex_variable_methods`
///
/// # Examples
///
/// ```
/// use a_fractal_a_day::functions::*;
/// use a_fractal_a_day::numbers::{Cplx, ComplexFunction};
/// let f: ComplexFunction = Box::new(|x| x*x);
///
/// assert_eq!(Cplx::new(9., 0.), f(Cplx::new(3., 0.)));
/// assert!((Cplx::new(6., 0.) - derivative(&f, &Cplx::new(3., 0.))).norm() < 1e-2);
/// ```
pub fn derivative(f: &ComplexFunction, z: &Cplx) -> Cplx {
    const H: Real = 1e-4;
    (f(z + H) - f(z - H)) / (2. * H)
}

pub fn random_formula<T: Rng>(rng: &mut T) -> Formula {
    // use up to 4 terms but at least 1
    let num_terms = (rng.gen_range(0f64, 1.) * 3.).floor() as i32 + 1;
    let mut terms: Vec<ComplexFunction> = Vec::new();
    let mut term_string: Vec<String> = Vec::new();

    let a_real_gen = |generator: &mut T| ((generator.gen_range(-1f64, 1.) * 3. * 10.).round() / 10.) as Real;
    let a_comp_gen = |generator: &mut T| Cplx::new(a_real_gen(generator), a_real_gen(generator));

    let mut possible_terms = Terms::new();
    // chance that all coefficients will be real
    let always_real = rng.gen_range(0f64, 1.) < 0.5;

    for _ in 0..num_terms {
        // let a be a complex number in 30% of all cases
        let a = if !always_real && rng.gen_range(0f64, 1.) < 0.3 {
                    Coef::Complex(a_comp_gen(rng))
                } else {
                    Coef::Real(a_real_gen(rng))
                };

        let neo = possible_terms.choice(a, rng);
        terms.push(neo.callable);
        term_string.push(neo.readable);
    }

    let f = move |x| terms.iter()
                          .map(move |f| f(x))
                          .fold(Cplx::new(0., 0.), |sum, x| sum + x);

    Formula {callable: Box::new(f),
             readable: "z ↦ ".to_string() + &term_string.join(" + ")}
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

    pub fn choice<T: Rng>(&mut self, a: Coef, rng: &mut T) -> Formula {
        let num = self.candidates_real.len();
        let idx = rng.gen_range(0, num as usize);
        match a {
            Coef::Real(x) => {self.candidates_comp.swap_remove(idx); self.candidates_real.swap_remove(idx)(x)},
            Coef::Complex(z) => {self.candidates_real.swap_remove(idx); self.candidates_comp.swap_remove(idx)(z)},
        }
    }
}
