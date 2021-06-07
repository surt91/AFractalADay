use log::info;

use rand::Rng;

use crate::{color::HSV, histogram::BoundsTypes};
use crate::fractal::FractalBuilder;
use super::{OdeFractal, OdeSystem, OdeTypes, random_normal};

use crate::numbers::Real;

use serde::{self, Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThomasOde {
    state: Vec<Real>,
    b: Real,
}

impl ThomasOde {
    fn new(
        x0: Option<Vec<Real>>,
        b: Option<Real>,
    ) -> ThomasOde {
        ThomasOde {
            state: x0.unwrap_or(vec![1., 1., 4.]),
            b: b.unwrap_or(0.208186),
        }
    }
}

impl OdeSystem for ThomasOde {
    fn get_state(&self) -> &Vec<Real>{
        &self.state
    }

    fn set_state(&mut self, state: Vec<Real>) {
        self.state = state;
    }

    fn derivative(&self, state: &[Real]) -> Vec<Real> {
        if let [x, y, z] = state {
            let mut out = vec![0.; 3];

            out[0] = y.sin() - self.b*x;
            out[1] = z.sin() - self.b*y;
            out[2] = x.sin() - self.b*z;

            out
        } else {
            unreachable!()
        }
    }
}

impl FractalBuilder
{
    pub fn thomas(self) -> OdeFractal {
        let mut rng = self.seed_rng();

        // critical value, below which there is chaotic behavior
        let b = 0.208186 - rng.gen_range(0.005, 0.02);
        let ode = ThomasOde::new(None, Some(b));
        let ode = OdeTypes::Thomas(ode);

        let color = HSV(rng.gen(), 1., 1.).to_rgb();

        let normal = random_normal(&mut rng);

        let gamma = self.gamma.unwrap_or(4.);
        let vibrancy = self.vibrancy.unwrap_or_else(|| rng.gen());
        let bounds = self.bounds.unwrap_or_else(|| BoundsTypes::StrictBounds);

        let description = format!("Thomas' cyclically symmetric attractor");

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
