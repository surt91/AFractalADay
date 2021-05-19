use log::info;

use rand::Rng;

use crate::color::HSV;
use crate::fractal::FractalBuilder;
use crate::histogram::BoundsTypes;
use super::{OdeFractal, OdeSystem, OdeTypes};

use crate::numbers::Real;

use serde::{self, Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DoublePendulumOde {
    state: Vec<Real>,
    // masses positions
    m1: Real,
    m2: Real,
    l1: Real,
    l2: Real,
}

impl DoublePendulumOde {
    fn new(
        state0: Option<Vec<Real>>,
        m1: Option<Real>,
        m2: Option<Real>,
        l1: Option<Real>,
        l2: Option<Real>,
    ) -> DoublePendulumOde {
        DoublePendulumOde {
            state: state0.unwrap_or_else(|| vec![1.7, 3.1415, 0., 0.]),
            m1: m1.unwrap_or(2.),
            m2: m2.unwrap_or(1.),
            l1: l1.unwrap_or(10.),
            l2: l2.unwrap_or(8.),
        }
    }
}

impl OdeSystem for DoublePendulumOde {
    fn get_dimension(&self) -> usize {
        4
    }

    fn get_state(&self) -> &Vec<Real>{
        &self.state
    }

    fn set_state(&mut self, state: Vec<Real>) {
        self.state = state;
    }

    fn derivative(&self, state: &[Real]) -> Vec<Real> {
        let g = 10.;
        // make some aliases to keep the equations short
        let m = self.m1 + self.m2;
        let m2 = self.m2;
        let l1 = self.l1;
        let l2 = self.l2;
        if let [theta1, theta2, theta_dot1, theta_dot2] = state {
            let mut out = vec![0.; 4];

            let d_theta1 = *theta_dot1;
            let d_theta2 = *theta_dot2;

            // cache often needed trigonometric functions
            let c = (theta1-theta2).cos();
            let s = (theta1-theta2).sin();

            let dd_theta1 = (m2*c*(l1*s*(theta_dot1*theta_dot1) - g*theta2.sin()) + m2*l2*s*(theta_dot2*theta_dot2) + m*g*theta1.sin()) / (m2*l1*(c*c) - m*l1);
            let dd_theta2 = (m2*l2*c*s*(theta_dot2*theta_dot2)  + m*l1*s*(theta_dot1*theta_dot1) + m*g*c*theta1.sin() - m*g*theta2.sin()) / (m*l2 - m2*l2*(c*c));

            out[0] = d_theta1;
            out[1] = d_theta2;
            out[2] = dd_theta1;
            out[3] = dd_theta2;

            out
        } else {
            unreachable!()
        }
    }

    fn project(&self, _n: [Real; 3]) -> [Real; 2]
    {
        let x = self.l1*self.state[0].sin() + self.l2*self.state[1].sin();
        let y = self.l1*self.state[0].cos() + self.l2*self.state[1].cos();
        [x, y]
    }
}

impl FractalBuilder
{
    pub fn double_pendulum(self) -> OdeFractal {
        let mut rng = self.seed_rng();

        let ode = DoublePendulumOde::new(None, None, None, None, None);
        let ode = OdeTypes::DoublePendulum(ode);

        let color = HSV(rng.gen(), 1., 1.).to_rgb();

        let normal = [0., 0., 1.];

        let gamma = self.gamma.unwrap_or(4.);
        let vibrancy = self.vibrancy.unwrap_or_else(|| rng.gen());
        let strict_bounds = self.bounds.unwrap_or_else(|| BoundsTypes::StrictBounds);

        let description = format!("DoublePendulum attractor");

        info!("Will render {}", description);

        OdeFractal {
            rng,
            description,
            ode,
            color,
            normal,
            timestep: 0.0001,
            total_time: 3000.,
            replica: 1,
            strict_bounds,
            gamma,
            vibrancy,
        }
    }
}
