use log::info;

use super::{EscapeTimeFractal, Convergence};
use crate::fractal::{FractalBuilder, RngType};
use crate::numbers::Cplx;

use super::style::Stylable;
use crate::color;
use crate::numbers::ComplexFunction;


pub struct JuliaFractal {
    f: ComplexFunction,
    rng: RngType,
    pub description: String,
    max_count: u64
}

impl FractalBuilder {
    pub fn julia(self) -> JuliaFractal {
        let mut rng = self.seed_rng();

        // fill in defaults, if members are not given
        // most defaults will be random
        let f = match self.f {
            Some(x) => x,
            None => ComplexFunction::random(&mut rng)
        };

        // let style = match self.style {
        //     Some(x) => x,
        //     None => Style::random_style(&mut rng)
        // };

        let description = format!("Julia Fractal of {}", f.human_readable());

        info!("Will render {}", description);
        // FIXME: For Julia fractals the colors need to be normalized somehow

        JuliaFractal {
            f,
            description,
            rng,
            max_count: 1000
        }
    }
}

impl Stylable for JuliaFractal {
    fn style(&self, conv: &Convergence) -> color::HSV {
        let fractional = conv.value.norm().ln().ln() as f64;
        let c = if fractional.is_nan() {
            conv.count as f64
        } else {
            conv.count as f64 - fractional
        };
        let mut h = (c / 40.).sin().abs();
        let s = 1f64;
        let mut v = 1f64;

        if h > 1. || !h.is_finite() {
            v = 0.;
            h = 1.;
        }

        color::HSV(h, s, v)
    }

    fn style_name(&self) -> &str {
        "vibrant"
    }
}

impl EscapeTimeFractal for JuliaFractal {
    fn description(&self) -> &str {
        &self.description
    }

    fn iterate(&self, mut state: Cplx) -> Convergence {
        let mut ctr = 0;
        // threshold is 2^2, since we compare to the square of the norm
        // as soon as the norm is >= 2 it is sure to diverge
        let threshold = 1e8;

        // the canonical julia set is `f = z^2 + c`,
        // but here we will use some arbitrary function

        while {
            state = self.f.eval(state);
            ctr += 1;

            state.norm_sqr() < threshold && ctr < self.max_count && !state.re.is_nan() && !state.im.is_nan()
        } {}
        // Convergence {count: 1, value: Cplx::new((ctr as Real) + 1. - (2f32.ln() / state.norm()) / 2f32.ln(), 0.) / 1000.}
        Convergence {count: ctr as f64, value: state}
    }

    fn get_rng(&mut self) -> &mut RngType {
        &mut self.rng
    }
}
