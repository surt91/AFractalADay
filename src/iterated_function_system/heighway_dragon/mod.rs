extern crate std;
extern crate num;

extern crate rand;
use self::rand::Rng;

use numbers::{Real, Cplx};
use iterated_function_system::IteratedFunctionSystem;

use super::iterated_function_system_builder::IteratedFunctionSystemBuilder;

pub struct HeighwayDragon {
    rng: rand::StdRng,
    pub description: String,
    p: Cplx
}

fn f1(z: Cplx) -> Cplx {
    (1. + Cplx::new(0., 1.)) * z / 2.
}

fn f2(z: Cplx) -> Cplx {
    1. - (1. - Cplx::new(0., 1.)) * z / 2.
}

impl Iterator for HeighwayDragon {
    type Item = [Real; 2];

    fn next(&mut self) -> Option<[Real; 2]> {
        let r = self.rng.gen::<f32>();

        match r {
            x if x < 0.5 => self.p = f1(self.p),
            _ => self.p = f2(self.p)
        };

        Some([self.p.re, self.p.im])
    }
}

impl IteratedFunctionSystem for HeighwayDragon {
    fn description(&self) -> &str {
        &self.description
    }

    fn get_rng(&mut self) -> &mut rand::StdRng {
        &mut self.rng
    }
}


impl IteratedFunctionSystemBuilder {
    pub fn heighway_dragon(self) -> HeighwayDragon {
        let rng: rand::StdRng = match self.seed {
            Some(x) => { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) },
            None => rand::StdRng::new().unwrap()
        };

        let description = "Heighway Dragon".to_owned();

        info!("Will render {}", description);

        let p = Cplx::new(0., 0.);

        HeighwayDragon {
            description,
            rng,
            p
        }
    }
}
