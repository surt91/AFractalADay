use num::complex::Complex;

use serde::{Serialize, Deserialize};

use rand::Rng;
use rand::distributions::{Standard, Normal, Distribution};
use rand::seq::SliceRandom;

use crate::fmt;
use std::str::FromStr;
use std::num::ParseFloatError;
use itertools::Itertools;

// adjust precision here
pub type Real = f64;
pub type Cplx = Complex<Real>;

fn round_cplx(x: Real, y: Real) -> Cplx {
    Cplx::new((x*10.).round()/10., (y*10.).round()/10.)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Op {
    Unary(char),
    Binary(char),
    Constant(Cplx),
    Variable
}

impl Op {
    pub fn random_operand<T: Rng>(rng: &mut T) -> Op {
        if rng.gen_bool(0.5) {
            Op::Variable
        } else {
            Op::Constant(round_cplx(rng.gen(), rng.gen()))
        }
    }

    pub fn random_operator<T: Rng>(rng: &mut T) -> Op {
        let choices = [
            Op::Unary('s'),
            Op::Unary('c'),
            Op::Unary('t'),
            Op::Binary('+'),
            Op::Binary('-'),
            Op::Binary('*'),
            Op::Binary('/'),
            Op::Binary('^')
        ];
        choices.choose(rng).unwrap().clone()
    }

    pub fn random_binary_operator<T: Rng>(rng: &mut T) -> Op {
        let choices = [
            Op::Binary('+'),
            Op::Binary('-'),
            Op::Binary('*'),
            Op::Binary('/'),
            Op::Binary('^')
        ];
        choices.choose(rng).unwrap().clone()
    }

    pub fn random_unary_or_operand<T: Rng>(rng: &mut T) -> Op {
        let choices = [
            Op::Unary('s'),
            Op::Unary('c'),
            Op::Unary('t'),
            Op::Variable,
            Op::Constant(round_cplx(rng.gen(), rng.gen()))
        ];
        choices.choose(rng).unwrap().clone()
    }

    pub fn random<T: Rng>(rng: &mut T) -> Op {
        let choices = [
            Op::Unary('s'),
            Op::Unary('c'),
            Op::Unary('t'),
            Op::Binary('+'),
            Op::Binary('-'),
            Op::Binary('*'),
            Op::Binary('/'),
            Op::Binary('^'),
            Op::Variable,
            Op::Constant(round_cplx(rng.gen(), rng.gen()))
        ];
        choices.choose(rng).unwrap().clone()
    }
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

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Op::Variable => "z".to_string(),
            Op::Unary('s') => "sin".to_string(),
            Op::Unary('c') => "cos".to_string(),
            Op::Unary('t') => "tan".to_string(),
            Op::Binary('+') => "+".to_string(),
            Op::Binary('-') => "-".to_string(),
            Op::Binary('*') => "*".to_string(),
            Op::Binary('/') => "/".to_string(),
            Op::Binary('^') => "^".to_string(),
            Op::Constant(x) => format!("{}", x),
            _ => "???".to_string()
        };
        write!(f, "{}", s)
    }
}

fn parse_cplx(s: &str) -> Result<Cplx, ParseFloatError> {
    Ok(
        if s.contains('i') {
            if s.contains('-') {
                let mut s = s.trim_matches('i').split('-');
                Cplx::new(s.next().unwrap().parse::<Real>()?, -s.next().unwrap().parse::<Real>()?)
            } else {
                let mut s = s.trim_matches('i').split('+');
                Cplx::new(s.next().unwrap().parse::<Real>()?, s.next().unwrap().parse::<Real>()?)
            }
        } else {
            Cplx::new(s.parse::<Real>()?, 0.)
        }
    )
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ComplexFunction {
    RPN(Vec<Op>),
    Polynom(Vec<Cplx>)
}

impl ComplexFunction {
    pub fn rpn_from_string(s: &str) -> ComplexFunction {
        let v = s.split(' ')
            .map(|t| Op::from_str(t).unwrap())
            .collect::<Vec<Op>>();
        ComplexFunction::RPN(v)
    }

    pub fn eval(&self, z: Cplx) -> Cplx {
        match self {
            ComplexFunction::RPN(rpn) => ComplexFunction::eval_rpn(z, rpn),
            ComplexFunction::Polynom(poly) => ComplexFunction::eval_poly(z, poly)
        }
    }

    fn eval_rpn(z: Cplx, ops: &[Op]) -> Cplx {
        let mut stack: Vec<Cplx> = vec![];
        for op in ops {
            let result = match op {
                Op::Unary(c) => {
                    let arg = stack.pop().expect("too few arguments for unary operation");
                    match c {
                        's' => arg.sin(),
                        'c' => arg.cos(),
                        't' => arg.tan(),
                        _ => panic!("unexpected unary operator: '{}'", c)
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
                        _ => panic!("unexpected binary operator: '{}'", c)
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

    fn eval_poly(z: Cplx, poly: &[Cplx]) -> Cplx {
        // TODO, maybe something fast like horner schema?
        (0..poly.len()).map(|i| poly[i] * z.powf(i as Real)).sum()
    }

    pub fn random(mut rng: &mut impl Rng) -> ComplexFunction {
        if rng.gen::<bool>() {
            let normal = Normal::new(0.0, 3.0);
            let v = normal.sample(&mut rng).abs();
            let num_terms = v.ceil() as usize;
            let coefficients = rng.sample_iter(&Standard)
                .map(|(x, y)| round_cplx(x, y))
                .take(num_terms)
                .collect::<Vec<Cplx>>();
            ComplexFunction::Polynom(coefficients)
        } else {
            let mut stack = vec![Op::random_binary_operator(rng)];
            let mut needed = 2;

            while needed > 0 {
                let next = if stack.len() > 7 {
                    Op::random_operand(rng)
                } else {
                    Op::random(rng)
                };
                match next {
                    Op::Binary(_) => needed += 1,
                    Op::Unary(_) => (),
                    Op::Constant(_) | Op::Variable => needed -= 1
                }
                stack.push(next);
            }

            ComplexFunction::RPN(stack.into_iter().rev().collect())
        }
    }

    /// Calculates the derivative of f at z.
    ///
    /// `https://en.wikipedia.org/wiki/Numerical_differentiation#Complex_variable_methods`
    ///
    /// # Examples
    ///
    /// ```
    /// use a_fractal_a_day::numbers::{Cplx, ComplexFunction};
    /// let f = ComplexFunction::rpn_from_string("z z *");
    ///
    /// assert_eq!(Cplx::new(9., 0.), f.eval(Cplx::new(3., 0.)));
    /// assert!((Cplx::new(6., 0.) - f.derivative(&Cplx::new(3., 0.))).norm() < 1e-2);
    /// ```
    pub fn derivative(&self, z: &Cplx) -> Cplx {
        const H: Real = 1e-4;
        (self.eval(z + H) - self.eval(z - H)) / (2. * H)
    }

    pub fn human_readable(&self) -> String {
        match self {
            ComplexFunction::RPN(x) => x.iter().map(|op| format!("{}", op)).join(" "),
            ComplexFunction::Polynom(x) => x.iter().enumerate().map(|(n, c)| format!("({}) z^{}", c, n)).join(" + "),
        }
    }
}

impl fmt::Display for ComplexFunction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.human_readable())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
