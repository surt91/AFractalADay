use log::info;

use rand::Rng;

use crate::{color::HSV, histogram::BoundsTypes};
use crate::fractal::FractalBuilder;
use super::{OdeFractal, OdeSystem, OdeTypes, random_normal};

use crate::numbers::Real;

use serde::{self, Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RosslerOde {
    state: Vec<Real>,
    a: Real,
    b: Real,
    c: Real,
}

impl RosslerOde {
    fn new(
        x0: Option<Vec<Real>>,
        a: Option<Real>,
        b: Option<Real>,
        c: Option<Real>,
    ) -> RosslerOde {
        RosslerOde {
            state: x0.unwrap_or(vec![1., 2., 0.]),
            a: a.unwrap_or(0.2),
            b: b.unwrap_or(0.2),
            c: c.unwrap_or(5.7),
        }
    }
}

impl OdeSystem for RosslerOde {
    fn get_state(&self) -> &Vec<Real>{
        &self.state
    }

    fn set_state(&mut self, state: Vec<Real>) {
        self.state = state;
    }

    fn derivative(&self, state: &[Real]) -> Vec<Real> {
        if let [x, y, z] = state {
            let mut out = vec![0.; 3];

            out[0] = -(y+z);
            out[1] = x + self.a*y;
            out[2] = self.b + x*z - self.c*z;

            out
        } else {
            unreachable!()
        }
    }
}

impl FractalBuilder
{
    pub fn rossler(self) -> OdeFractal {
        let mut rng = self.seed_rng();

        let ode = RosslerOde::new(None, None, None, None);
        let ode = OdeTypes::Rossler(ode);

        let color = HSV(rng.gen(), 1., 1.).to_rgb();

        let normal = random_normal(&mut rng);

        let gamma = self.gamma.unwrap_or(4.);
        let vibrancy = self.vibrancy.unwrap_or_else(|| rng.gen());
        let bounds = self.bounds.unwrap_or_else(|| BoundsTypes::StrictBounds);

        let description = format!("Rössler attractor");

        info!("Will render {}", description);

        OdeFractal {
            rng,
            description,
            ode,
            color,
            normal,
            timestep: 0.01,
            total_time: 1000000.,
            replica: 10,
            bounds,
            gamma,
            vibrancy,
        }
    }
}
