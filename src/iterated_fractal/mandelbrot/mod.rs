extern crate std;
extern crate num;

extern crate rand;

use super::{IteratedFractal, Convergence};
use super::iterated_fractal_builder::IteratedFractalBuilder;
use numbers::Cplx;

use super::style::Stylable;
use color;

pub struct MandelbrotFractal {
    rng: rand::StdRng,
    pub description: String,
    max_count: u64
}

impl IteratedFractalBuilder {
    pub fn mandelbrot(self) -> MandelbrotFractal {
        let mut rng: rand::StdRng = match self.seed {
            Some(x) => { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) },
            None => rand::StdRng::new().unwrap()
        };

        let description = format!("Mandelbrot Fractal");

        info!("Will render {}", description);
        // FIXME: For Mandelbrot fractals the colors need to be normalized somehow

        MandelbrotFractal {
            description,
            rng,
            max_count: 100
        }
    }
}

impl Stylable for MandelbrotFractal {
    fn style(&self, conv: &Convergence) -> color::HSV {
        let fractional = conv.value.norm().ln().ln() as f64;
        let c = if fractional.is_nan() {
            conv.count as f64
        } else {
            conv.count as f64 - fractional
        };
        let mut h = c / (self.max_count - 1) as f64;
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

impl IteratedFractal for MandelbrotFractal {
    fn description(&self) -> &str {
        &self.description
    }

    fn iterate(&self, mut state: Cplx) -> Convergence {
        let mut ctr = 0;
        // threshold is 2^2, since we compare to the square of the norm
        // as soon as the norm is >= 2 it is sure to diverge
        let threshold = 4.;
        let start = state;

        while {
            // right now this is a julia set, I have to look up mandelbrot, but the wifi is bad
            state = state * state + start;
            ctr += 1;

            state.norm_sqr() < threshold && ctr < self.max_count && !state.re.is_nan() && !state.im.is_nan()
        } {}
        // Convergence {count: 1, value: Cplx::new((ctr as Real) + 1. - (2f32.ln() / state.norm()) / 2f32.ln(), 0.) / 1000.}
        Convergence {count: ctr as f64, value: state}
    }

    fn get_rng(&mut self) -> &mut rand::StdRng {
        &mut self.rng
    }
}
