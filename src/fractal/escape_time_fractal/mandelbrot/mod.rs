use rand::Rng;

use serde::{self, Serialize, Deserialize};

use log::info;

use std::f64::consts::PI;
use std::cmp::max;

use super::{EscapeTimeFractal, Convergence, EscapeTypes};
use crate::numbers::{Real, Cplx};
use crate::fractal::{FractalBuilder, RngType, default_rng};

use super::style::Stylable;
use crate::color;
use crate::colormap::Colormap;

fn default_colormap() -> Colormap {
    Colormap::viridis()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MandelbrotFractal {
    #[serde(skip)]
    #[serde(default = "default_rng")]
    rng: RngType,
    pub description: String,
    max_count: u64,
    shift: Cplx,
    zoom: Real,
    #[serde(default = "default_colormap")]
    colormap: Colormap,
}

impl FractalBuilder {
    pub fn mandelbrot(self) -> MandelbrotFractal {
        let mut rng = self.seed_rng();

        // guess a random point on the complex plane which could be interesting
        let extra_r = 1. + rng.gen_range(0.0, 0.1);
        let shift = if rng.gen::<f32>() < 0.5 {
            // either near the cardiod
            let phi = rng.gen_range(0., 2.*PI as Real);
            let r = (1. - phi.cos()) / 2.;
            let r = r * extra_r;  // go a bit outside
            let x = r * phi.cos() + 0.25;
            let y = r * phi.sin();
            Cplx::new(x, y)
        } else {
            // or near the circle
            let phi = rng.gen_range(0., 2.*PI as Real);
            let r = 0.25;
            let r = r * extra_r;  // go a bit outside
            let x = r * phi.cos() - 1.;
            let y = r * phi.sin();
            Cplx::new(x, y)
        };

        let zoom = 2u64.pow(rng.gen_range(0, 14));

        let colormap = Colormap::random(&mut rng);

        let description = format!("Mandelbrot Fractal at ~({:.2}), zoom {}x", shift, zoom);

        info!("Will render {}", description);
        // FIXME: For Mandelbrot fractals the colors need to be normalized somehow

        MandelbrotFractal {
            description,
            rng,
            max_count: max(1000, 4*zoom),
            shift,
            zoom: zoom as f32,
            colormap,
        }
    }
}

impl Stylable for MandelbrotFractal {
    // TODO: more and nicer styles
    fn style(&self, conv: &Convergence) -> color::HSV {
        let c = conv.count;
        let h = c / (self.max_count - 1) as f64;

        self.colormap.value(&h).to_hsv()
    }

    fn style_name(&self) -> &str {
        self.colormap.name()
    }
}

impl EscapeTimeFractal for MandelbrotFractal {
    fn description(&self) -> &str {
        &self.description
    }

    fn iterate(&self, mut state: Cplx) -> Convergence {
        let mut ctr = 0;
        // threshold is 2^2, since we compare to the square of the norm
        // as soon as the norm is >= 2 it is sure to diverge
        let threshold = 4.;
        state = state / self.zoom + self.shift;
        let start = state;

        while {
            state = state * state + start;
            ctr += 1;

            state.norm_sqr() < threshold && ctr < self.max_count && !state.re.is_nan() && !state.im.is_nan()
        } {}
        Convergence {count: ctr as f64, value: state}
    }

    fn get_rng(&mut self) -> &mut RngType {
        &mut self.rng
    }

    fn get_serializable(&self) -> EscapeTypes {
        EscapeTypes::Mandelbrot(self.clone())
    }
}
