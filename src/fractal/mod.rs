mod escape_time_fractal;
mod iterated_function_system;
mod lsystem;
mod lattice;
mod quality;

// reexport configuration types
pub use self::escape_time_fractal::style::Style;
pub use self::escape_time_fractal::EscapeTypes;
pub use self::iterated_function_system::variation::Variation;
pub use self::iterated_function_system::transformation::{Transformation, AffineTransformation};
pub use self::iterated_function_system::symmetry::Symmetry;
pub use self::iterated_function_system::fractal_flame::FractalFlame;
pub use self::iterated_function_system::IterationFractalType;
pub use self::lsystem::{Alphabet, Lrules, LSystem};
pub use self::lattice::{LatticeFractal, Ising};

use rand_pcg::Pcg32;

use serde_json;

use log::info;

use std::io;
use crate::png_helper::save_png;

use crate::FractalType;
use crate::numbers::{Coef, ComplexFunction};

use rand::{Rng, SeedableRng, FromEntropy};
use rand::rngs::SmallRng;

pub type RngType = Pcg32;

fn default_rng() -> RngType {
    RngType::seed_from_u64(SmallRng::from_entropy().gen::<u64>())
}

enum FractalInstance {
    EscapeTime(Box<dyn escape_time_fractal::EscapeTimeFractal>),
    IFS(Box<dyn iterated_function_system::IteratedFunctionSystem>),
    LSys(Box<dyn lsystem::LSystem>),
    Lattice(Box<dyn lattice::LatticeFractal>),
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
    seed: Option<u64>,
    variation: Option<Variation>,
    post_transform: Option<Transformation>,
    final_transform: Option<Variation>,
    symmetry: Option<Symmetry>,
    vibrancy: Option<f64>,
    gamma: Option<f64>,

    // for escape time
    a: Option<Coef>,
    f: Option<ComplexFunction>,
    style: Option<Style>,

    // for mandelbrot
    zoom: Option<u64>,
    center: Option<(f64, f64)>,

    // for L systems
    iterations: Option<u32>,
    start: Option<Vec<Alphabet>>,
    rules: Option<Lrules>,
    angle: Option<f64>,

    // for lattices
    dimensions: Option<(u32, u32)>,
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

            zoom: None,
            center: None,

            iterations: None,
            start: None,
            rules: None,
            angle: None,

            dimensions: None,
        }
    }
    pub fn seed_rng(&self) -> RngType {
        match self.seed {
            Some(x) => RngType::seed_from_u64(x),
            None => RngType::seed_from_u64(SmallRng::from_entropy().gen::<u64>())
        }
    }

    pub fn seed(mut self, seed: usize) -> FractalBuilder {
        self.seed = Some(seed as u64);
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

    pub fn formula(mut self, f: ComplexFunction) -> FractalBuilder {
        self.f = Some(f);
        self
    }

    pub fn rpn(mut self, rpn: &Option<String>) -> FractalBuilder {
        self.f = rpn.clone().and_then(|x| Some(ComplexFunction::rpn_from_string(&x)));
        self
    }

    pub fn zoom(mut self, zoom: &Option<u64>) -> FractalBuilder {
        self.zoom = zoom.clone();
        self
    }

    pub fn center(mut self, center: &Option<(f64, f64)>) -> FractalBuilder {
        self.center = center.clone();
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

    pub fn dimensions(mut self, dimensions: &Option<(u32, u32)>) -> FractalBuilder {
        self.dimensions = dimensions.clone();
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
            FractalType::Ising => FractalInstance::Lattice(Box::new(self.ising())),
            FractalType::QuadraticMap => FractalInstance::IFS(Box::new(self.quadratic_map())),
            FractalType::Lorenz => FractalInstance::IFS(Box::new(self.lorenz())),
            FractalType::DoublePendulum => FractalInstance::IFS(Box::new(self.double_pendulum())),
            FractalType::Random => unreachable!(),
            // FIXME This has to be replaced by a better approach
            FractalType::LoadJson(ref json) => {
                let out: FractalInstance;
                let ifs = FractalBuilder::ifs_from_json(&json);
                let lsys = FractalBuilder::lsys_from_json(&json);
                let escape_type = FractalBuilder::escape_type_from_json(&json);

                if ifs.is_ok() {
                    out = FractalInstance::IFS(Box::new(ifs.unwrap()))
                } else if lsys.is_ok() {
                    out = FractalInstance::LSys(Box::new(lsys.unwrap()))
                } else if escape_type.is_ok() {
                    out = match escape_type.unwrap() {
                        EscapeTypes::Mandelbrot(x) => FractalInstance::EscapeTime(Box::new(x)),
                        EscapeTypes::Newton(x) => FractalInstance::EscapeTime(Box::new(x)),
                        EscapeTypes::None => panic!("invalid json")
                    }
                } else {
                    panic!("invalid json");
                }
                out
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
                f.suggested_iterations(),
                f.suggested_parallelism(),
                supersampling
            ),
            FractalInstance::LSys(ref mut f) => f.render(resolution, None, None),
            FractalInstance::Lattice(ref mut f) => f.render(resolution, None, None),
        };

        let (x, y) = resolution;
        save_png(filename, x, y, &buffer)?;

        Ok(good)
    }

    /// same as render, but faster and lower quality
    pub fn render_draft(&mut self, resolution: (u32, u32), filename: &str) -> io::Result<bool> {
        let (buffer, good) = match self.fractal {
            FractalInstance::EscapeTime(ref mut f) => f.render(resolution, None, None),
            FractalInstance::IFS(ref mut f) => f.render(
                resolution,
                f.suggested_iterations_draft(),
                f.suggested_parallelism(),
                false
            ),
            FractalInstance::LSys(ref mut f) => f.render(resolution, None, None),
            FractalInstance::Lattice(ref mut f) => f.render(resolution, None, None),
        };

        let (x, y) = resolution;
        save_png(filename, x, y, &buffer)?;

        Ok(good)
    }

    pub fn description(&self) -> &str {
        match self.fractal {
            FractalInstance::EscapeTime(ref f) => f.description(),
            FractalInstance::IFS(ref f) => f.description(),
            FractalInstance::LSys(ref f) => f.description(),
            FractalInstance::Lattice(ref f) => f.description(),
        }
    }

    pub fn json(&self) -> String {
        match self.fractal {
            FractalInstance::EscapeTime(ref f) => {
                serde_json::to_string(&f.get_serializable()).expect(&format!("Escape: {:#?}", &f.get_serializable()))
            },
            FractalInstance::IFS(ref f) => {
                serde_json::to_string(&f.get_serializable()).expect(&format!("IFS: {:#?}", &f.get_serializable()))
            },
            FractalInstance::LSys(ref f) => {
                serde_json::to_string(&f.get_serializable()).expect(&format!("Lsys: {:#?}", &f.get_serializable()))
            },
            FractalInstance::Lattice(ref f) => {
                serde_json::to_string(&f.get_serializable()).expect(&format!("Lsys: {:#?}", &f.get_serializable()))
            },
        }
    }

    pub fn estimate_quality_before(&mut self) -> bool {
        match self.fractal_type {
            FractalType::FractalFlame | FractalType::QuadraticMap => {
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

        let f1_config = match f1.get_serializable() {
            IterationFractalType::IFS(x) => x,
            _ => return Err(())
        };
        let f2_config = match f2.get_serializable() {
            IterationFractalType::IFS(x) => x,
            _ => return Err(())
        };

        fn count_trafo(c: &FractalFlame) -> usize {
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

use crate::color::{RGBA, HSV, color_variance};

pub fn estimate_quality_after(rgb: &[RGBA], _resolution: &(u32, u32)) -> bool {
    let hsv: Vec<HSV> = rgb.iter().map(|c| c.blend_black().to_hsv()).collect();
    let var = color_variance(&hsv);
    info!("variance: {:.3}", var);

    var > 0.01
}

#[test]
fn test_json() {
    let f = FractalBuilder::new().build(&FractalType::RandomLSystem);
    println!("Lsystem: {:?}", f);
    let json = f.json();
    println!("json: {:?}", json);
    let g: lsystem::Generic = serde_json::from_str(&json).unwrap();
    println!("Lsystem: {:?}", g);
    println!("Lsystem: {:?}", g.description());
}
