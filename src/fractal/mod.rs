mod escape_time_fractal;
mod iterated_function_system;
mod lsystem;
mod quality;

// reexport configuration types
pub use self::escape_time_fractal::style::Style;
pub use self::iterated_function_system::variation::Variation;
pub use self::iterated_function_system::transformation::Transformation;
pub use self::iterated_function_system::symmetry::Symmetry;
pub use self::iterated_function_system::serialize::IteratedFunctionSystemConfig;
pub use self::lsystem::{Alphabet, Lrules, LSystem};

extern crate serde_json;

use std;
use std::io;
use png;

use FractalType;
use numbers::{Coef, Formula};

use rand::{Rng, SeedableRng, Isaac64Rng, FromEntropy};
use rand::rngs::SmallRng;

pub type RngType = Isaac64Rng;
pub type SeedType = [u8; 32];

enum FractalInstance {
    EscapeTime(Box<escape_time_fractal::EscapeTimeFractal>),
    IFS(Box<iterated_function_system::IteratedFunctionSystem>),
    LSys(Box<lsystem::LSystem>),
}

pub struct Fractal {
    fractal: FractalInstance,
    fractal_type: FractalType,
}

impl std::fmt::Debug for Fractal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.fractal_type)
    }
}

pub struct FractalBuilder {
    // for iterated function systems
    seed: Option<SeedType>,
    variation: Option<Variation>,
    post_transform: Option<Transformation>,
    final_transform: Option<Variation>,
    symmetry: Option<Symmetry>,
    vibrancy: Option<f64>,
    gamma: Option<f64>,

    // for escape time
    a: Option<Coef>,
    f: Option<Formula>,
    style: Option<Style>,

    // for L systems
    iterations: Option<u32>,
    start: Option<Vec<Alphabet>>,
    rules: Option<Lrules>,
    angle: Option<f64>,
}

impl FractalBuilder {
    pub fn new() -> FractalBuilder {
        FractalBuilder {
            seed: None,
            variation: None,
            post_transform: None,
            final_transform: None,
            symmetry: None,
            vibrancy: None,
            gamma: None,

            a: None,
            f: None,
            style: None,

            iterations: None,
            start: None,
            rules: None,
            angle:None,
        }
    }
    pub fn seed_rng(&self) -> RngType {
        match self.seed {
            Some(x) => RngType::from_seed(x),
            None => RngType::from_seed(SmallRng::from_entropy().gen::<SeedType>())
        }
    }

    pub fn seed(mut self, seed: usize) -> FractalBuilder {
        self.seed = Some(Isaac64Rng::new_from_u64(seed as u64).gen::<SeedType>());
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

    pub fn iterations(mut self, iterations: &Option<u32>) -> FractalBuilder {
        self.iterations = iterations.clone();
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

    pub fn start(mut self, start: &Option<Vec<Alphabet>>) -> FractalBuilder {
        self.start = start.clone();
        self
    }

    pub fn rules(mut self, rules: &Option<Lrules>) -> FractalBuilder {
        self.rules = rules.clone();
        self
    }

    pub fn angle(mut self, angle: &Option<f64>) -> FractalBuilder {
        self.angle = angle.clone();
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
            FractalType::KochCurve => FractalInstance::LSys(Box::new(self.koch_curve())),
            FractalType::SierpinskiArrowhead => FractalInstance::LSys(Box::new(self.sierpinski_arrowhead())),
            FractalType::HilbertCurve => FractalInstance::LSys(Box::new(self.hilbert_curve())),
            FractalType::GosperCurve => FractalInstance::LSys(Box::new(self.gosper_curve())),
            FractalType::Bush => FractalInstance::LSys(Box::new(self.bush())),
            FractalType::PenroseTiling => FractalInstance::LSys(Box::new(self.penrose_tiling())),
            FractalType::Pentigree => FractalInstance::LSys(Box::new(self.pentigree())),
            FractalType::Tritile => FractalInstance::LSys(Box::new(self.tritile())),
            FractalType::LDragon => FractalInstance::LSys(Box::new(self.ldragon())),
            FractalType::RandomLSystem => FractalInstance::LSys(Box::new(self.generic())),
            FractalType::Random => unreachable!(),
            // FIXME This has to be replaced by a better approach
            FractalType::LoadJson(ref json) => {
                self.ifs_from_json(&json)
                    .and_then(|x| Ok(FractalInstance::IFS(Box::new(x))))
                    .or_else::<Fractal, _>(|_| Ok(FractalInstance::LSys(Box::new(FractalBuilder::lsys_from_json(&json).unwrap()))))
                    .expect("invalid json")
            },
        };

        Fractal {
            fractal: instance,
            fractal_type: fractal_type.clone(),
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
                                                            supersampling
                                                        ),
            FractalInstance::LSys(ref mut f) => f.render(resolution, None, None),
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
                                                            false
                                                        ),
            FractalInstance::LSys(ref mut f) => f.render(resolution, None, None),
        };

        let (x, y) = resolution;
        png::save_png(filename, x, y, &buffer)?;

        Ok(good)
    }

    pub fn description(&self) -> &str {
        match self.fractal {
            FractalInstance::EscapeTime(ref f) => f.description(),
            FractalInstance::IFS(ref f) => f.description(),
            FractalInstance::LSys(ref f) => f.description()
        }
    }
    pub fn json(&self) -> String {
        match self.fractal {
            FractalInstance::EscapeTime(ref _f) => "todo".to_owned(),
            FractalInstance::IFS(ref f) => serde_json::to_string(&f.get_serializable()).unwrap(),
            FractalInstance::LSys(ref f) => serde_json::to_string(&f.get_serializable()).unwrap(),
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

    pub fn combine(&self, other: &Fractal) -> Result<Fractal, ()> {
        // both need to be IFS
        let (f1, f2) = match (&self.fractal, &other.fractal) {
            (&FractalInstance::IFS(ref a), &FractalInstance::IFS(ref b)) => (a, b),
            _ => return Err(()),
        };

        let f1_config = f1.get_serializable();
        let f2_config = f2.get_serializable();

        use self::iterated_function_system::AffineTransformation;
        use self::iterated_function_system::Transformation;

        fn count_trafo(c: &IteratedFunctionSystemConfig) -> usize {
            c.transformations.iter().filter(
                |x| match x {
                    &&Transformation::Affine(
                        AffineTransformation{
                            symmetry: x,
                            ..
                        }
                    ) => !x,
                    _ => true
                }
            )
            .count()
        }

        // count trafos excluding symmetries
        let f1_num_trafo = count_trafo(&f1_config);
        let f2_num_trafo = count_trafo(&f2_config);

        // create the new config
        let mut f_config = f1_config.clone();
        let mut rng = SmallRng::from_entropy();

        // TODO: take one trafos from f2 and add it to f, may overwrite
        let f2_chosen_trafo = rng.gen_range(0, f2_num_trafo);
        if rng.gen::<f64>() < 0.5 {
            // overwrite
            let overwrite = rng.gen_range(0, f1_num_trafo);
            f_config.colors[overwrite] = f2_config.colors[f2_chosen_trafo].clone();
            f_config.transformations[overwrite] = f2_config.transformations[f2_chosen_trafo].clone();
        } else {
            // add
            let overwrite = rng.gen_range(0, f1_num_trafo);
            let new_prob = f_config.probabilities[overwrite].clone();
            let previous_prob = if overwrite > 0 {f_config.probabilities[overwrite-1]} else {0.};
            let reduced_old_prob = (previous_prob + new_prob) / 2.;
            f_config.probabilities[overwrite] = reduced_old_prob;
            f_config.probabilities.insert(overwrite+1, new_prob);
            f_config.colors.insert(overwrite+1, f2_config.colors[f2_chosen_trafo].clone());
            f_config.transformations.insert(overwrite+1, f2_config.transformations[f2_chosen_trafo].clone());
        }

        // TODO: add a symmetry from f1 or f2 (if applicable)


        // FIXME ugly detour over json string
        let json = serde_json::to_string(&f_config).unwrap();
        Ok(FractalBuilder::new().build(&FractalType::LoadJson(json)))
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

pub fn estimate_quality_after(rgb: &[RGBA], _resolution: &(u32, u32)) -> bool {
    let hsv: Vec<HSV> = rgb.iter().map(|c| c.blend_black().to_hsv()).collect();
    let var = color_variance(&hsv);
    info!("variance: {:.3}", var);

    var > 0.01
}
