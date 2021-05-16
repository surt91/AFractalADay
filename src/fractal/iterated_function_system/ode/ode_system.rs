use std::fmt;
use crate::numbers::Real;


pub trait OdeSystem : Sync + Send + fmt::Debug {
    fn get_dimension(&self) -> usize;
    fn get_state(&self) -> &Vec<Real>;
    fn set_state(&mut self, state: Vec<Real>);
    fn derivative(&self, state: &[Real]) -> Vec<Real>;

    fn update(&mut self) {
        let next = self.rk4_step(None);
        self.set_state(next)
    }

    fn rk4_step(&self, tau: Option<Real>) -> Vec<Real> {
        let tau = tau.unwrap_or_else(|| 0.0001);
        let state = self.get_state();
        let mut tmp = vec![0.; self.get_dimension()];

        let est0 = self.derivative(&state);

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
}
