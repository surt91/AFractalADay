extern crate std;
extern crate num;

extern crate rand;
use self::rand::Rng;

use numbers::Cplx;
use iterated_function_system::IteratedFunctionSystem;

use super::iterated_function_system_builder::IteratedFunctionSystemBuilder;

pub struct HeighwayDragon {
    iterations: usize,
    rng: rand::StdRng,
    pub description: String,
}

fn f1(z: Cplx) -> Cplx {
    (1. + Cplx::new(0., 1.)) * z / 2.
}

fn f2(z: Cplx) -> Cplx {
    1. - (1. - Cplx::new(0., 1.)) * z / 2.
}

impl IteratedFunctionSystem for HeighwayDragon {
    fn description(&self) -> &str {
        &self.description
    }

    fn iterate(&mut self) -> Vec<Cplx>{
        let mut p1 = Cplx::new(0., 0.);
        let mut p2 = Cplx::new(1., 0.);

        self.rng.gen_iter::<f32>()
                .take(self.iterations)
                .map(|r| {
                    match r {
                        x if x < 0.25 => { p1 = f1(p1); p1 },
                        x if x < 0.5 => { p1 = f2(p1); p1 },
                        x if x < 0.75 => { p2 = f1(p2); p2 },
                        _ => { p2 = f2(p2); p2 },
                    }
                }).collect()
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

        let iterations = match self.iterations {
            Some(x) => x,
            None => 1000000
        };


        let description = "Heighway Dragon".to_owned();

        info!("Will render {}", description);

        HeighwayDragon {
            description,
            rng,
            iterations
        }
    }
}
