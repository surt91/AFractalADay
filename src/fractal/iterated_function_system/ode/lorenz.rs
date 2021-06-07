use log::info;

use rand::Rng;

use crate::{color::HSV, histogram::BoundsTypes};
use crate::fractal::FractalBuilder;
use super::{OdeFractal, OdeSystem, OdeTypes, random_normal};

use crate::numbers::Real;

use serde::{self, Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LorenzOde {
    state: Vec<Real>,
    sigma: Real,
    b: Real,
    r: Real,
}

impl LorenzOde {
    fn new(
        x0: Option<Vec<Real>>,
        sigma: Option<Real>,
        r: Option<Real>,
        b: Option<Real>,
    ) -> LorenzOde {
        LorenzOde {
            state: x0.unwrap_or(vec![1., 1., 20.]),
            sigma: sigma.unwrap_or(10.),
            r: r.unwrap_or(28.),
            b: b.unwrap_or(8./3.),
        }
    }
}

impl OdeSystem for LorenzOde {
    fn get_state(&self) -> &Vec<Real>{
        &self.state
    }

    fn set_state(&mut self, state: Vec<Real>) {
        self.state = state;
    }

    fn derivative(&self, state: &[Real]) -> Vec<Real> {
        if let [x, y, z] = state {
            let mut out = vec![0.; 3];

            out[0] = self.sigma*(y-x);
            out[1] = self.r*x - y - x*z;
            out[2] = x*y - self.b*z;

            out
        } else {
            unreachable!()
        }
    }
}

impl FractalBuilder
{
    pub fn lorenz(self) -> OdeFractal {
        let mut rng = self.seed_rng();

        let ode = LorenzOde::new(None, None, None, None);
        let ode = OdeTypes::Lorenz(ode);

        let color = HSV(rng.gen(), 1., 1.).to_rgb();

        let normal = random_normal(&mut rng);

        let gamma = self.gamma.unwrap_or(4.);
        let vibrancy = self.vibrancy.unwrap_or_else(|| rng.gen());
        let bounds = self.bounds.unwrap_or_else(|| BoundsTypes::StrictBounds);

        let description = format!("Lorenz attractor");

        info!("Will render {}", description);

        OdeFractal {
            rng,
            description,
            ode,
            color,
            normal,
            timestep: 0.0001,
            total_time: 100.,
            replica: 1,
            bounds,
            gamma,
            vibrancy,
        }
    }
}
