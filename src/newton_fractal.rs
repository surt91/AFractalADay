extern crate std;
extern crate num;

extern crate rand;

use iterated_fractal::{IteratedFractal, Convergence};
use functions::{Coef, Cplx, ComplexFunction, Formula, derivative, random_formula};

use style::Style;


pub struct NewtonFractalBuilder {
    a: Option<Coef>,
    f: Option<Formula>,
    seed: Option<usize>,
    style: Option<Style>
}

// Builder Pattern to create a NewtonFractal
impl NewtonFractalBuilder {
    pub fn new() -> NewtonFractalBuilder {
        NewtonFractalBuilder {
            a: None,
            f: None,
            seed: None,
            style: None
        }
    }

    pub fn coefficient(mut self, a: Coef) -> NewtonFractalBuilder {
        self.a = Some(a);
        self
    }

    pub fn formula(mut self, f: Formula) -> NewtonFractalBuilder {
        self.f = Some(f);
        self
    }

    pub fn seed(mut self, seed: usize) -> NewtonFractalBuilder {
        self.seed = Some(seed);
        self
    }

    pub fn style(mut self, style: Style) -> NewtonFractalBuilder {
        self.style = Some(style);
        self
    }

    pub fn build(self) -> NewtonFractal {
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
            None => NewtonFractal::random_coef(&mut rng)
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

        NewtonFractal {
            a,
            f: f.callable,
            description,
            rng,
            style
        }
    }
}

pub struct NewtonFractal {
    a: Coef,
    f: ComplexFunction,
    rng: rand::StdRng,
    pub description: String,
    style: Style
}

impl IteratedFractal for NewtonFractal {
    fn iterate(&self, mut state: Cplx) -> Convergence {
        let mut ctr = 0;
        let threshold = 1e-4;
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
            ctr += 1;

            (state - tmp).norm() > threshold && ctr < 1000 && !state.re.is_nan() && !state.im.is_nan()
        } {}
        Convergence {count: ctr, value: state}
    }

    fn get_rng(&mut self) -> &mut rand::StdRng {
        &mut self.rng
    }

    fn get_style(&self) -> &Style {
        &self.style
    }
}
