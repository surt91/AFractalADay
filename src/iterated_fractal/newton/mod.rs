extern crate std;
extern crate num;

extern crate rand;
use self::rand::Rng;

use super::{IteratedFractal, Convergence};
use super::iterated_fractal_builder::IteratedFractalBuilder;
use numbers::{Coef, Cplx, ComplexFunction};
use self::functions::{derivative, random_formula, random_coef};

use iterated_fractal::style::{Style, Stylable};
use color;

mod functions;

pub struct NewtonFractal {
    a: Coef,
    f: ComplexFunction,
    rng: rand::StdRng,
    pub description: String,
    style: Style,
    random_color: f64,
    random_count: f64
}

impl IteratedFractalBuilder {
    pub fn newton(self) -> NewtonFractal {
        let mut rng: rand::StdRng = match self.seed {
            Some(x) => { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) },
            None => rand::StdRng::new().unwrap()
        };

        // fill in defaults, if members are not given
        // most defaults will be random
        let f = match self.f {
            Some(x) => x,
            None => random_formula(&mut rng)
        };

        let a = match self.a {
            Some(x) => x,
            None => random_coef(&mut rng)
        };

        let style = match self.style {
            Some(x) => x,
            None => Style::random_style(&mut rng)
        };


        let mut description = match a {
            Coef::Real(x) if (1. - x).abs() < 1e-4 => "Newton Fractal of ".to_string(),
            Coef::Real(x) => format!("Generalized Newton Fractal (x = {}) of ", x),
            Coef::Complex(y) => format!("Generalized Newton Fractal (x = {}) of ", y)
        };
        description += &f.readable;

        info!("Will render {}", description);
        info!("use style '{}'", style);

        // use randomness to determine the colors
        let random_color = rng.gen_range(0f64, 1.);
        let random_count = rng.gen_range(0f64, 1.);

        info!("rcol {}", random_color);
        info!("rcnt {}", random_count);

        NewtonFractal {
            a,
            f: f.callable,
            description,
            rng,
            style,
            random_color,
            random_count
        }
    }
}

impl Stylable for NewtonFractal {
    fn style(&self, conv: &Convergence) -> color::HSV {
        (self.style.callable)(conv, Some(self.random_color), Some(self.random_count))
    }

    fn style_name(&self) -> &str {
        &self.style.readable
    }
}

impl IteratedFractal for NewtonFractal {
    fn iterate(&self, mut state: Cplx) -> Convergence {
        let mut ctr = 0.;
        let threshold = 1e-12;
        let mut tmp;

        let kernel: Box<Fn(Cplx) -> Cplx> = match self.a {
            Coef::Complex(z) => Box::new(move |state| state - z * (self.f)(state) / derivative(&self.f, &state)),
            Coef::Real(x) => Box::new(move |state| state - x * (self.f)(state) / derivative(&self.f, &state))
        };
        // this is a do while loop, mind that the "body" is actually the
        // condition and the body is empty, thus omitted
        while {
            tmp = state;
            state = kernel(state);
            ctr += 1.;

            (state - tmp).norm_sqr() > threshold && ctr < 1000. && !state.re.is_nan() && !state.im.is_nan()
        } {}
        // for smooth color, add a normalized distance
        if (state - tmp).norm_sqr() < threshold {
            ctr += (state - tmp).norm_sqr() / threshold;
        }
        Convergence {count: ctr as f64, value: state}
    }

    fn get_rng(&mut self) -> &mut rand::StdRng {
        &mut self.rng
    }
}
