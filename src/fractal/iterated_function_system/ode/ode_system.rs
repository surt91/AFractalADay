use std::fmt;

use crate::numbers::Real;

use quaternion;

pub trait OdeSystem : Sync + Send + fmt::Debug {
    fn get_state(&self) -> &Vec<Real>;
    fn set_state(&mut self, state: Vec<Real>);
    fn derivative(&self, state: &[Real]) -> Vec<Real>;

    fn get_dimension(&self) -> usize {
        3
    }

    // default projection is 3D to random plane
    // for non 3D ODE, you need a custom projection
    fn project(&self, normal: [Real; 3]) -> [Real; 2] {
        let state = self.get_state();
        let point = [state[0], state[1], state[2]];

        let quat = quaternion::rotation_from_to([0., 0., 1.], normal);
        let p = quaternion::rotate_vector(quat, point);

        [p[0], p[1]]
    }

    fn update(&mut self, tau: Real) {
        // let tau = self.adaptive_tau();
        let next = self.rk4_step(self.get_state(), tau);
        self.set_state(next)
    }

    fn rk4_step(&self, state: &[Real], tau: Real) -> Vec<Real> {
        let mut tmp = vec![0.; self.get_dimension()];

        let est0 = self.derivative(state);

        for i in 0..self.get_dimension() {
            tmp[i] = state[i] + tau/2.*est0[i];
        }
        let est1 = self.derivative(&tmp);

        for i in 0..self.get_dimension() {
            tmp[i] = state[i] + tau/2.*est1[i];
        }
        let est2 = self.derivative(&tmp);

        for i in 0..self.get_dimension() {
            tmp[i] = state[i] + tau*est2[i];
        }
        let est3 = self.derivative(&tmp);

        for i in 0..self.get_dimension() {
            tmp[i] = state[i] + tau / 6.*(
                est0[i] + 2.*est1[i] + 2.*est2[i] + est3[i]
            );
        }

        tmp
    }

    fn adaptive_tau(&self, tau: Real) -> Real {
        // target precision
        let desired = 1e-6;
        // safety
        let s = 0.9;
        // maximum tau
        let tau_max = 0.01;

        let state1 = self.get_state();
        let state2 = self.get_state();

        let state1 = self.rk4_step(state1, tau);
        let state2 = self.rk4_step(state2, tau/2.);
        let state2 = self.rk4_step(&state2, tau/2.);

        let mut tmp1 = 0.;
        let mut tmp2 = 0.;
        for i in 0..self.get_dimension() {
            tmp1 += (state2[i]-state1[i])*(state2[i]-state1[i]);
            tmp2 += (state1[i]+state2[i])*(state1[i]+state2[i])/4.;
        }
        let r = tmp1.sqrt() / tmp2.sqrt();

        let mut tau_new = s * tau * (desired/r).powf(1./5.);
        // ensure that tau changes not too fast
        tau_new = tau_new.clamp(tau / 4., tau * 4.).clamp(tau / 4., tau_max);

        tau_new
    }
}
