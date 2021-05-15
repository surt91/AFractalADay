use crate::color::RGB;
use crate::numbers::Real;
use rand::Rng;
use super::{Perturbable, Samplable};

pub struct QuadraticMapSampler<T>
    where T: Rng
{
    pub rng: T,
    pub a: Vec<Real>,
    pub color: RGB,
    pub p: [Real; 2],
}

impl<T> Iterator for QuadraticMapSampler<T>
    where T: Rng
{
    type Item = ([Real; 2], RGB);

    fn next(&mut self) -> Option<([Real; 2], RGB)> {

        let a = &self.a;
        let [x, y] = self.p;
        self.p[0] = a[0] + a[1]*x + a[2]*x*x + a[3]*x*y + a[4]*y + a[5]*y*y;
        self.p[1] = a[6] + a[7]*x + a[8]*x*x + a[9]*x*y + a[10]*y + a[11]*y*y;

        Some((self.p, self.color.clone()))
    }
}


impl<T> Perturbable for QuadraticMapSampler<T>
    where T: Rng
{
    fn perturb(&mut self) {
        self.p[0] += self.rng.gen_range(-0.01, 0.01)
    }
}

impl<T> Samplable for QuadraticMapSampler<T>
    where T: Rng {}