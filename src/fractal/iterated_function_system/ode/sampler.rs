use crate::color::RGB;
use crate::numbers::Real;
use rand::Rng;
use super::{Perturbable, Samplable, OdeSystem};

pub struct OdeFractalSampler<T>
    where T: Rng
{
    pub rng: T,
    pub ode: Box<dyn OdeSystem>,
    pub color: RGB,
    pub normal: [Real; 3],

    pub tau: Real,
}

impl<T> Iterator for OdeFractalSampler<T>
    where T: Rng
{
    type Item = ([Real; 2], RGB);

    fn next(&mut self) -> Option<([Real; 2], RGB)> {
        self.ode.update(self.tau);

        let p = self.ode.project(self.normal);

        Some((p, self.color.clone()))
    }
}


impl<T> Perturbable for OdeFractalSampler<T>
    where T: Rng
{
    fn perturb(&mut self) {
        let mut state = self.ode.get_state().clone();
        for x in state.iter_mut() {
            *x += self.rng.gen_range(-0.01, 0.01);
        }
        self.ode.set_state(state);
    }
}

impl<T> Samplable for OdeFractalSampler<T>
    where T: Rng {}
