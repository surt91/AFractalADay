extern crate std;
extern crate num;

extern crate rand;
use self::rand::Rng;

use numbers::Real;
use iterated_function_system::IteratedFunctionSystem;

use super::iterated_function_system_builder::IteratedFunctionSystemBuilder;

pub struct BarnsleyFern {
    rng: rand::StdRng,
    pub description: String,
    p: [Real; 2]
}

/// calculates A*x+b, for a 2x2 Matrix A
fn affine_transformation(a: [[Real; 2]; 2], b: [Real; 2], x: [Real; 2]) -> [Real; 2]
{
    let mut out = [0.; 2];
    out[0] = a[0][0] * x[0] + a[0][1] * x[1] + b[0];
    out[1] = a[1][0] * x[0] + a[1][1] * x[1] + b[1];
    out
}

fn f1(x: [Real; 2]) -> [Real; 2] {
    let a = [[0., 0.], [0., 0.16]];
    let b = [0., 0.];
    affine_transformation(a, b, x)
}

fn f2(x: [Real; 2]) -> [Real; 2] {
    let a = [[0.85, 0.04], [-0.04, 0.85]];
    let b = [0., 1.6];
    affine_transformation(a, b, x)
}

fn f3(x: [Real; 2]) -> [Real; 2] {
    let a = [[0.2, -0.26], [0.23, 0.22]];
    let b = [0., 1.6];
    affine_transformation(a, b, x)
}

fn f4(x: [Real; 2]) -> [Real; 2] {
    let a = [[-0.15, 0.28], [0.26, 0.24]];
    let b = [0., 0.44];
    affine_transformation(a, b, x)
}

impl Iterator for BarnsleyFern {
    type Item = [Real; 2];

    fn next(&mut self) -> Option<[Real; 2]> {
        let r = self.rng.gen::<f32>();

        match r {
            x if x < 0.01 => { self.p = f1(self.p); self.p },
            x if x < 0.86 => { self.p = f2(self.p); self.p },
            x if x < 0.93 => { self.p = f3(self.p); self.p },
            _ => { self.p = f4(self.p); self.p },
        };

        Some(self.p)
    }
}

impl IteratedFunctionSystem for BarnsleyFern {
    fn description(&self) -> &str {
        &self.description
    }

    fn get_rng(&mut self) -> &mut rand::StdRng {
        &mut self.rng
    }
}


impl IteratedFunctionSystemBuilder {
    pub fn barnsley_fern(self) -> BarnsleyFern {
        let rng: rand::StdRng = match self.seed {
            Some(x) => { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) },
            None => rand::StdRng::new().unwrap()
        };

        let description = "Barnsley Fern".to_owned();

        info!("Will render {}", description);

        let p = [0., 0.];

        BarnsleyFern {
            description,
            rng,
            p
        }
    }
}
