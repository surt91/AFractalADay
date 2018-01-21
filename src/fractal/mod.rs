pub mod escape_time_fractal;
pub mod iterated_function_system;

extern crate serde_json;

extern crate rand;
use self::rand::{Rng, SeedableRng};

pub type RngType = rand::Isaac64Rng;
pub type SeedType = [u64; 4];

use FractalType;

// for iterated function systems
use self::iterated_function_system::variation::Variation;
use self::iterated_function_system::symmetry::Symmetry;

// for escape time
use numbers::{Coef, Formula};
use self::escape_time_fractal::style::Style;

use std::io;

pub enum FractalInstance {
    EscapeTime(Box<escape_time_fractal::EscapeTimeFractal>),
    IFS(Box<iterated_function_system::IteratedFunctionSystem>)
}

pub struct Fractal {
    pub fractal: FractalInstance,
    pub fractal_type: FractalType
}

pub struct FractalBuilder {
    // for iterated function systems
    seed: Option<SeedType>,
    variation: Option<Variation>,
    symmetry: Option<Symmetry>,

    // for escape time
    a: Option<Coef>,
    f: Option<Formula>,
    style: Option<Style>
}

impl FractalBuilder {
    pub fn new() -> FractalBuilder {
        FractalBuilder {
            seed: None,
            variation: None,
            symmetry: None,

            a: None,
            f: None,
            style: None
        }
    }
    pub fn seed_rng(&self) -> RngType {
        match self.seed {
            Some(x) => RngType::from_seed(&x),
            None => RngType::from_seed(&rand::weak_rng().gen::<SeedType>())
        }
    }

    pub fn seed(mut self, seed: usize) -> FractalBuilder {
        let s = [seed];
        self.seed = Some(rand::StdRng::from_seed(&s).gen::<SeedType>());
        self
    }

    pub fn variation(mut self, variation: &Option<Variation>) -> FractalBuilder {
        self.variation = variation.clone();
        self
    }

    pub fn symmetry(mut self, symmetry: &Option<Symmetry>) -> FractalBuilder {
        self.symmetry = symmetry.clone();
        self
    }

    pub fn coefficient(mut self, a: Coef) -> FractalBuilder {
        self.a = Some(a);
        self
    }

    pub fn formula(mut self, f: Formula) -> FractalBuilder {
        self.f = Some(f);
        self
    }

    pub fn style(mut self, style: &Option<Style>) -> FractalBuilder {
        self.style = style.clone();
        self
    }

    pub fn build(self, fractal_type: &FractalType) -> Fractal {
        let instance = match *fractal_type {
            FractalType::Newton => FractalInstance::EscapeTime(Box::new(self.newton())),
            FractalType::Julia => FractalInstance::EscapeTime(Box::new(self.julia())),
            FractalType::Mandelbrot => FractalInstance::EscapeTime(Box::new(self.mandelbrot())),
            FractalType::HeighwayDragon => FractalInstance::IFS(Box::new(self.heighway_dragon())),
            FractalType::BarnsleyFern => FractalInstance::IFS(Box::new(self.barnsley_fern())),
            FractalType::SierpinskiGasket => FractalInstance::IFS(Box::new(self.sierpinski_gasket())),
            FractalType::SierpinskiPentagon => FractalInstance::IFS(Box::new(self.sierpinski_pentagon())),
            FractalType::PythagoreanTree => FractalInstance::IFS(Box::new(self.pythagorean_tree())),
            FractalType::AppolonianGasket => FractalInstance::IFS(Box::new(self.appolonian_gasket())),
            FractalType::MobiusFlame => FractalInstance::IFS(Box::new(self.mobius_flame())),
            FractalType::FractalFlame => FractalInstance::IFS(Box::new(self.fractal_flame())),
            FractalType::LoadJson(ref json) => FractalInstance::IFS(Box::new(self.from_json(&json))),
            FractalType::Random => unreachable!()
        };

        Fractal {
            fractal: instance,
            fractal_type: fractal_type.clone()
        }
    }
}

impl Fractal {
    pub fn render(&mut self, resolution: (u32, u32), filename: &str) -> io::Result<f64> {
        match self.fractal {
            FractalInstance::EscapeTime(ref mut f) => f.render(resolution, None, None, filename),
            FractalInstance::IFS(ref mut f) => f.render(resolution, 1000, filename)
        }
    }

    pub fn description(&self) -> &str {
        match self.fractal {
            FractalInstance::EscapeTime(ref f) => f.description(),
            FractalInstance::IFS(ref f) => f.description()
        }
    }
    pub fn json(&self) -> String {
        match self.fractal {
            FractalInstance::EscapeTime(ref _f) => "todo".to_owned(),
            FractalInstance::IFS(ref f) => serde_json::to_string_pretty(&f.get_serializable()).unwrap()
        }
    }

    pub fn estimate_quality(&mut self) -> bool {
        match self.fractal_type {
            FractalType::FractalFlame => {
                match self.fractal {
                    FractalInstance::IFS(ref mut f) => f.estimate_quality(),
                    _ => unreachable!(),
                }
            },
            _ => true,
        }
    }
}