use rand::Rng;

use serde::{self, Serialize, Deserialize};

use log::info;

use super::{EscapeTimeFractal, Convergence, EscapeTypes};
use crate::fractal::{FractalBuilder, RngType, default_rng};
use crate::numbers::{Coef, Cplx, ComplexFunction};
use super::style::{Style, Stylable, style_serialize, style_deserialize};
use crate::color;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewtonFractal {
    a: Coef,
    f: ComplexFunction,
    #[serde(skip)]
    #[serde(default = "default_rng")]
    rng: RngType,
    pub description: String,
    #[serde(serialize_with = "style_serialize", deserialize_with = "style_deserialize")]
    style: Style,
    random_color: f64,
    random_count: f64
}

impl FractalBuilder {
    pub fn newton(self) -> NewtonFractal {
        let mut rng = self.seed_rng();

        // fill in defaults, if members are not given
        // most defaults will be random
        let f = match self.f {
            Some(x) => x,
            None => ComplexFunction::random(&mut rng)
        };

        let a = match self.a {
            Some(x) => x,
            None => Coef::random(&mut rng)
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
        description += &f.human_readable();

        info!("Will render {}", description);
        info!("use style '{}'", style);

        // use randomness to determine the colors
        let random_color = rng.gen_range(0f64, 1.);
        let random_count = rng.gen_range(0f64, 1.);

        info!("rcol {}", random_color);
        info!("rcnt {}", random_count);

        NewtonFractal {
            a,
            f,
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

impl EscapeTimeFractal for NewtonFractal {
    fn description(&self) -> &str {
        &self.description
    }

    fn iterate(&self, mut state: Cplx) -> Convergence {
        let mut ctr = 0.;
        let threshold = 1e-12;
        let mut tmp;

        let kernel: Box<dyn Fn(Cplx) -> Cplx> = match self.a {
            Coef::Real(x) if (1. - x).abs() < 1e-4 => Box::new(move |state| state - self.f.eval(state) / self.f.derivative(&state)),
            Coef::Real(x) => Box::new(move |state| state - x * self.f.eval(state) / self.f.derivative(&state)),
            Coef::Complex(z) => Box::new(move |state| state - z * self.f.eval(state) / self.f.derivative(&state)),
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

    fn get_rng(&mut self) -> &mut RngType {
        &mut self.rng
    }

    fn get_serializable(&self) -> EscapeTypes {
        EscapeTypes::Newton(self.clone())
    }
}
