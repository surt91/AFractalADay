mod escape_time_fractal;
mod iterated_function_system;
mod quality;

// reexport configuration types
pub use self::escape_time_fractal::style::Style;
pub use self::iterated_function_system::variation::Variation;
pub use self::iterated_function_system::symmetry::Symmetry;

extern crate serde_json;

use std::io;
use png;

use FractalType;
use numbers::{Coef, Formula};

use rand::{self, Rng, SeedableRng};

pub type RngType = rand::Isaac64Rng;
pub type SeedType = [u64; 4];

enum FractalInstance {
    EscapeTime(Box<escape_time_fractal::EscapeTimeFractal>),
    IFS(Box<iterated_function_system::IteratedFunctionSystem>)
}

pub struct Fractal {
    fractal: FractalInstance,
    fractal_type: FractalType,

    vibrancy: f64,
    gamma: f64,
}

pub struct FractalBuilder {
    // for iterated function systems
    seed: Option<SeedType>,
    variation: Option<Variation>,
    symmetry: Option<Symmetry>,
    vibrancy: Option<f64>,
    gamma: Option<f64>,

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
            vibrancy: None,
            gamma: None,

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

    pub fn vibrancy(mut self, vibrancy: &Option<f64>) -> FractalBuilder {
        self.vibrancy = vibrancy.clone();
        self
    }

    pub fn gamma(mut self, gamma: &Option<f64>) -> FractalBuilder {
        self.gamma = gamma.clone();
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
        let vibrancy = self.vibrancy.unwrap_or(0.9);
        let gamma = self.gamma.unwrap_or(4.);

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
            fractal_type: fractal_type.clone(),
            vibrancy,
            gamma,
        }
    }
}

impl Fractal {
    pub fn render(&mut self, resolution: (u32, u32), filename: &str, supersampling: bool) -> io::Result<bool> {
        let (buffer, good) = match self.fractal {
            FractalInstance::EscapeTime(ref mut f) => f.render(resolution, None, None),
            FractalInstance::IFS(ref mut f) => f.render(
                                                            resolution,
                                                            1000,
                                                            self.vibrancy,
                                                            self.gamma,
                                                            supersampling
                                                        )
        };

        let (x, y) = resolution;
        png::save_png(filename, x, y, &buffer)?;

        Ok(good)
    }

    /// same as render, but faster and lower quality
    pub fn render_draft(&mut self, resolution: (u32, u32), filename: &str) -> io::Result<bool> {
        let (buffer, good) = match self.fractal {
            FractalInstance::EscapeTime(ref mut f) => f.render(resolution, None, None),
            FractalInstance::IFS(ref mut f) => f.render(
                                                            resolution,
                                                            100,
                                                            self.vibrancy,
                                                            self.gamma,
                                                            false
                                                        )
        };

        let (x, y) = resolution;
        png::save_png(filename, x, y, &buffer)?;

        Ok(good)
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
            FractalInstance::IFS(ref f) => serde_json::to_string(&f.get_serializable()).unwrap()
        }
    }

    pub fn estimate_quality_before(&mut self) -> bool {
        match self.fractal_type {
            FractalType::FractalFlame => {
                match self.fractal {
                    FractalInstance::IFS(ref mut f) => f.estimate_quality_before(),
                    _ => unreachable!(),
                }
            },
            _ => true,
        }
    }
}

pub fn render_wrapper(
                        fractal: &mut Fractal,
                        filename: &str,
                        dim: &(u32, u32),
                        supersampling: bool
                     )
                     -> (bool, String, String)
{
    // for some fractals, we can estimate if it will look good
    // so abort, if not before rendering
    if ! fractal.estimate_quality_before() {
        return (false, "".to_string(), "".to_string())
    }

    let finished = fractal.render(*dim, filename, supersampling)
                          .expect("creation of fractal failed");

    let description = fractal.description().to_owned();
    info!("{}", description);

    let json = fractal.json();

    // TODO: we need something better than the variance to estimate the
    // quality of an image, maybe do an FFT and look for intermediate frequencies?

    let finished = match fractal.fractal_type {
          FractalType::FractalFlame
        | FractalType::MobiusFlame
        | FractalType::Newton
        | FractalType::Mandelbrot
        | FractalType::Julia
          => finished,
        _ => true
    };

    (finished, description, json)
}

pub fn render_draft(
                        fractal: &mut Fractal,
                        filename: &str,
                        dim: &(u32, u32),
                   )
                     -> String
{
    // for some fractals, we can estimate if it will look good
    // so abort, if not before rendering
    if ! fractal.estimate_quality_before() {
        return "".to_string()
    }

    fractal.render_draft(*dim, filename)
           .expect("creation of fractal failed");

    let json = fractal.json();

    json
}

use color::{RGBA, HSV, color_variance};

pub fn estimate_quality_after(rgb: &[RGBA], resolution: &(u32, u32)) -> bool {
    let hsv: Vec<HSV> = rgb.iter().map(|c| c.blend_black().to_hsv()).collect();
    let var = color_variance(&hsv);
    info!("variance: {:.3}", var);

    var > 0.01
}
