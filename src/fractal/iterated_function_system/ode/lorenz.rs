use log::info;

use rand::Rng;

use crate::{color::HSV, histogram::BoundsTypes};
use crate::fractal::FractalBuilder;
use super::{OdeFractal, OdeSystem, OdeTypes, random_normal};

use crate::numbers::Real;

use serde::{self, Serialize, Deserialize};

use quaternion;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LorenzOde {
    state: Vec<Real>,
    sigma: Real,
    b: Real,
    r: Real,

    tau: Real,
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

            tau: 0.000001,
        }
    }
}

impl OdeSystem for LorenzOde {
    fn get_dimension(&self) -> usize {
        3
    }

    fn get_tau(&self) -> Real {
        self.tau
    }

    fn set_tau(&mut self, tau: Real) {
        self.tau = tau;
    }

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

    fn project(&self, n: [Real; 3]) -> [Real; 2]
    {
        let point = [self.state[0], self.state[1], self.state[2]];

        let quat = quaternion::rotation_from_to([0., 0., 1.], n);
        let p = quaternion::rotate_vector(quat, point);

        [p[0], p[1]]
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

        let gamma = match self.gamma {
            Some(s) => s,
            None => 4.
        };

        let vibrancy = match self.vibrancy {
            Some(s) => s,
            None => rng.gen()
        };

        let strict_bounds = BoundsTypes::StrictBounds;

        let description = format!("Lorenz attractor");

        info!("Will render {}", description);

        OdeFractal {
            rng,
            description,
            ode,
            color,
            normal,
            strict_bounds,
            gamma,
            vibrancy,
        }
    }
}
