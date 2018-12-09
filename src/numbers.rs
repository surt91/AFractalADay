use rand::Rng;
extern crate num;
use self::num::complex::Complex;

use fmt;
use std::str::FromStr;
use std::num::ParseFloatError;

// adjust precision here
pub type Real = f32;
pub type Cplx = Complex<Real>;

#[derive(Debug, Serialize, Deserialize)]
pub enum Op {
    Unary(char),
    Binary(char),
    Constant(Cplx),
    Variable
}

impl FromStr for Op {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = match s {
            "z" | "x" => Op::Variable,
            "sin" => Op::Unary('s'),
            "cos" => Op::Unary('c'),
            "tan" => Op::Unary('t'),
            "+" => Op::Binary('+'),
            "-" => Op::Binary('-'),
            "*" => Op::Binary('*'),
            "/" => Op::Binary('/'),
            "^" => Op::Binary('^'),
            _ => Op::Constant(parse_cplx(s)?),
        };
        Ok(op)
    }
}

fn parse_cplx(s: &str) -> Result<Cplx, ParseFloatError> {
    Ok(
        if s.contains("i") {
            if s.contains("-") {
                let mut s = s.trim_matches('i').split("-");
                Cplx::new(s.next().unwrap().parse::<Real>()?, -s.next().unwrap().parse::<Real>()?)
            } else {
                let mut s = s.trim_matches('i').split("+");
                Cplx::new(s.next().unwrap().parse::<Real>()?, s.next().unwrap().parse::<Real>()?)
            }
        } else {
            Cplx::new(s.parse::<Real>()?, 0.)
        }
    )
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ComplexFunction {
    RPN(Vec<Op>),
    Polynom(Vec<Cplx>)
}

impl ComplexFunction {
    pub fn rpn_from_string(s: &str) -> ComplexFunction {
        let v = s.split(" ").map(|t| Op::from_str(t).unwrap()).collect::<Vec<Op>>();
        ComplexFunction::RPN(v)
    }

    pub fn eval(&self, z: Cplx) -> Cplx {
        match self {
            ComplexFunction::RPN(rpn) => ComplexFunction::eval_rpn(z, rpn),
            ComplexFunction::Polynom(poly) => ComplexFunction::eval_poly(z, poly)
        }
    }

    fn eval_rpn(z: Cplx, ops: &Vec<Op>) -> Cplx {
        let mut stack: Vec<Cplx> = vec![];
        for op in ops {
            let result = match op {
                Op::Unary(c) => {
                    let arg = stack.pop().expect("too few arguments for unary operation");
                    match c {
                        's' => arg.sin(),
                        'c' => arg.cos(),
                        't' => arg.tan(),
                        _ => panic!(format!("unexpected unary operator: '{}'", c))
                    }
                }
                Op::Binary(c) => {
                    let x = stack.pop().expect("missing first argument for binary operation");
                    let y = stack.pop().expect("missing second argument for binary operation");
                    match c {
                        '+' => x + y,
                        '-' => x - y,
                        '*' => x * y,
                        '/' => x / y,
                        '^' => x.powc(y),
                        _ => panic!(format!("unexpected binary operator: '{}'", c))
                    }
                }
                Op::Variable => z,
                Op::Constant(x) => *x
            };
            stack.push(result);
        }
        assert!(stack.len() == 1);
        stack[0]
    }

    fn eval_poly(z: Cplx, poly: &Vec<Cplx>) -> Cplx {
        // TODO, maybe something fast like horner schema?
        (0..poly.len()).map(|i| poly[i] * z.powf(i as Real)).sum()
    }

    pub fn random<T: Rng>(rng: &mut T) -> ComplexFunction {
        let rpn = "z cos 1. +";
        ComplexFunction::rpn_from_string(rpn)
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
    pub fn derivative(&self, z: &Cplx) -> Cplx {
        const H: Real = 1e-4;
        (self.eval(z + H) - self.eval(z - H)) / (2. * H)
    }

    pub fn human_readable(&self) -> &str {
        " "
    }
}

impl fmt::Display for ComplexFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.human_readable())
    }
}

#[derive(Debug)]
pub enum Coef {
    Real(Real),
    Complex(Cplx)
}

impl Coef {
    pub fn random<T: Rng>(rng: &mut T) -> Coef {
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
}

impl fmt::Display for Coef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
