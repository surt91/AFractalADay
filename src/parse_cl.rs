use a_fractal_a_day::histogram::BoundsTypes;
use clap::{App, Arg, ArgGroup};

use std::f64::consts::PI;
use std::fmt;
use std::fs;
use std::io::prelude::*;

use crate::FractalType;
use crate::fractal::{Style, Variation, Symmetry, Lrules};

#[derive(Debug)]
pub struct Options {
    pub seed: Option<usize>,
    pub filename: Option<String>,
    pub style: Option<Style>,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub tweet: bool,
    pub quiet: bool,
    pub optipng: bool,
    pub supersampling: bool,
    pub bounds: Option<BoundsTypes>,
    pub fractal_type: FractalType,
    pub variation: Option<Variation>,
    pub symmetry: Option<Symmetry>,
    pub vibrancy: Option<f64>,
    pub gamma: Option<f64>,
    pub iterations: Option<u32>,
    pub rules: Option<Lrules>,
    pub angle: Option<f64>,
    pub rpn: Option<String>,
    pub zoom: Option<u64>,
    pub center: Option<(f64, f64)>,
    pub qmaprule: Option<String>,
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Options: type: {}, seed: {}, name:  {}, style: {}, variation: {}, tweet: {}, quiet: {}, optipng: {}",
                  self.fractal_type,
                  self.seed.map_or("random".to_string(), |s| s.to_string()),
                  self.filename.as_ref().unwrap_or(&"random".to_string()),
                  self.style.as_ref().and_then(|s| Some(s.name())).unwrap_or("random".to_string()),
                  self.variation.as_ref().and_then(|v| Some(v.name())).unwrap_or("default".to_string()),
                  self.tweet,
                  self.quiet,
                  self.optipng,
              )
    }
}

// FIXME: return result with an clError, instead of panicing
pub fn parse_cl() -> Options {
    let matches = App::new(env!("CARGO_PKG_NAME"))
              .version(env!("CARGO_PKG_VERSION"))
              .about(env!("CARGO_PKG_DESCRIPTION"))
              .author(env!("CARGO_PKG_AUTHORS"))
              .arg(Arg::with_name("tweet")
                    .short("t")
                    .long("tweet")
                    .help("do tweet the generated image")
              )
              .arg(Arg::with_name("seed")
                    .long("seed")
                    .takes_value(true)
                    .help("the seed for the random number generator ")
              )
              .arg(Arg::with_name("filename")
                    .short("f")
                    .long("filename")
                    .takes_value(true)
                    .help("the name of the outputted image")
              )
              .arg(Arg::with_name("style")
                    .short("s")
                    .long("style")
                    .takes_value(true)
                    .help("the name of the style applied to visualize")
              )
              .arg(Arg::with_name("height")
                    .short("y")
                    .long("height")
                    .takes_value(true)
                    .help("the height of the output image in px")
              )
              .arg(Arg::with_name("width")
                    .short("x")
                    .long("width")
                    .takes_value(true)
                    .help("the width of the output image in px")
              )
              .arg(Arg::with_name("supersampling")
                    .long("supersampling")
                    .help("sample the fractal at 4x the resolution")
              )
              .arg(Arg::with_name("quiet")
                    .short("q")
                    .long("quiet")
                    .help("do only print error messages")
              )
              .arg(Arg::with_name("no-optipng")
                    .short("n")
                    .long("no-optipng")
                    .help("do not minify the result with optipng")
              )
              .arg(Arg::with_name("newton")
                    .long("newton")
                    .help("render a newton fractal")
                    .group("escape_time")
              )
              .arg(Arg::with_name("julia")
                    .long("julia")
                    .help("render a julia fractal")
                    .group("escape_time")
              )
              .arg(Arg::with_name("mandelbrot")
                    .long("mandelbrot")
                    .help("render a mandelbrot fractal")
                    .group("escape_time")
              )
              .arg(Arg::with_name("dragon")
                    .long("dragon")
                    .help("render a dragon curve")
                    .group("iterated_function_system")
              )
              .arg(Arg::with_name("fern")
                    .long("fern")
                    .help("render a Barnsley fern")
                    .group("iterated_function_system")
              )
              .arg(Arg::with_name("sierpinski")
                    .long("sierpinski")
                    .help("render a Sierpinski gasket")
                    .group("iterated_function_system")
              )
              .arg(Arg::with_name("sierpinski-pentagon")
                    .long("sierpinski-pentagon")
                    .help("render a Sierpinski pentagon")
                    .group("iterated_function_system")
              )
              .arg(Arg::with_name("pythagorean")
                    .long("pythagorean")
                    .help("render a Pythagorean tree")
                    .group("iterated_function_system")
              )
              .arg(Arg::with_name("appolonian")
                    .long("appolonian")
                    .help("render an Appolonian gasket")
                    .group("iterated_function_system")
              )
              .arg(Arg::with_name("mobius")
                    .long("mobius")
                    .help("render a mobius flame")
                    .group("iterated_function_system")
              )
              .arg(Arg::with_name("flame")
                    .long("flame")
                    .help("render a fractal flame")
                    .group("iterated_function_system")
              )
              .arg(Arg::with_name("json")
                    .long("json")
                    .help("load a fractal from a file (only ifs)")
                    .takes_value(true)
                    .group("iterated_function_system")
              )
              .arg(Arg::with_name("kochcurve")
                    .long("kochcurve")
                    .help("render a Koch curve")
                    .group("lsystem")
              )
              .arg(Arg::with_name("sierpinskiarrowhead")
                    .long("sierpinskiarrowhead")
                    .help("render a Sierpinski arrowhead")
                    .group("lsystem")
              )
              .arg(Arg::with_name("hilbertcurve")
                    .long("hilbertcurve")
                    .help("render a Hilbert curve")
                    .group("lsystem")
              )
              .arg(Arg::with_name("gospercurve")
                    .long("gospercurve")
                    .help("render a Gosper curve")
                    .group("lsystem")
              )
              .arg(Arg::with_name("bush")
                    .long("bush")
                    .help("render a bush")
                    .group("lsystem")
              )
              .arg(Arg::with_name("penrose")
                    .long("penrose")
                    .help("render a Penrose tiling")
                    .group("lsystem")
              )
              .arg(Arg::with_name("pentigree")
                    .long("pentigree")
                    .help("render a Pentigree")
                    .group("lsystem")
              )
              .arg(Arg::with_name("tritile")
                    .long("tritile")
                    .help("render a Triangular tiling")
                    .group("lsystem")
              )
              .arg(Arg::with_name("ldragon")
                    .long("ldragon")
                    .help("render a Dragon curve as L-system")
                    .group("lsystem")
              )
              .arg(Arg::with_name("random_lsystem")
                    .long("lsystem")
                    .help("render a random lsystem")
                    .group("lsystem")
              )
              .arg(Arg::with_name("ising")
                    .long("ising")
                    .help("render a critical Ising system")
                    .group("lattice")
              )
              .arg(Arg::with_name("quadraticmap")
                    .long("quadraticmap")
                    .help("render a quadratic map")
                    .group("qmap")
              )
              .arg(Arg::with_name("lorenz")
                    .long("lorenz")
                    .help("render the Lorenz attractor")
                    .group("ode")
              )
              .arg(Arg::with_name("rossler")
                    .long("rossler")
                    .help("render the Rössler attractor")
                    .group("ode")
              )
              .arg(Arg::with_name("thomas")
                    .long("thomas")
                    .help("render the Thomas attractor")
                    .group("ode")
              )
              .arg(Arg::with_name("doublependulum")
                    .long("doublependulum")
                    .help("render the double pendulum (fractal dimension 2, I guess)")
                    .group("ode")
              )
              .group(ArgGroup::with_name("iterated_function_system")
                  .conflicts_with("escape_time")
                  .conflicts_with("lsystem")
                  .conflicts_with("lattice")
                  .conflicts_with("qmap")
                  .conflicts_with("ode")
              )
              .group(ArgGroup::with_name("escape_time")
                  .conflicts_with("iterated_function_system")
                  .conflicts_with("lsystem")
                  .conflicts_with("lattice")
                  .conflicts_with("qmap")
                  .conflicts_with("ode")
              )
              .group(ArgGroup::with_name("lsystem")
                  .conflicts_with("iterated_function_system")
                  .conflicts_with("escape_time")
                  .conflicts_with("lattice")
                  .conflicts_with("qmap")
                  .conflicts_with("ode")
              )
              .group(ArgGroup::with_name("lattice")
                  .conflicts_with("iterated_function_system")
                  .conflicts_with("escape_time")
                  .conflicts_with("lsystem")
                  .conflicts_with("qmap")
                  .conflicts_with("ode")
              )
              .group(ArgGroup::with_name("qmap")
                  .conflicts_with("iterated_function_system")
                  .conflicts_with("escape_time")
                  .conflicts_with("lsystem")
                  .conflicts_with("lattice")
                  .conflicts_with("ode")
              )
              .group(ArgGroup::with_name("ode")
                  .conflicts_with("iterated_function_system")
                  .conflicts_with("escape_time")
                  .conflicts_with("lsystem")
                  .conflicts_with("lattice")
                  .conflicts_with("qmap")
              )
              .group(ArgGroup::with_name("symmetry")
                  .conflicts_with("escape_time")
                  .conflicts_with("lsystem")
                  .conflicts_with("lattice")
                  .conflicts_with("qmap")
              )
              .group(ArgGroup::with_name("iterated_fractals")
                  .conflicts_with("escape_time")
                  .conflicts_with("lsystem")
                  .conflicts_with("lattice")
              )
              .arg(Arg::with_name("mirror")
                    .long("mirror")
                    .help("creates a vertical mirror symmetry in the resulting fractal")
                    .group("symmetry")
                    .requires("iterated_function_system")
              )
              .arg(Arg::with_name("mirror-horizontal")
                    .long("mirror-horizontal")
                    .help("creates a horizontal mirror symmetry in the resulting fractal")
                    .group("symmetry")
                    .requires("iterated_function_system")
              )
              .arg(Arg::with_name("no-symmetry")
                    .long("no-symmetry")
                    .help("creates a fractal without artificial symmetries")
                    .group("symmetry")
                    .requires("iterated_function_system")
              )
              .arg(Arg::with_name("rotational")
                    .long("rotational")
                    .takes_value(true)
                    .help("creates a fractal with an x-fold rotational symmetry")
                    .group("symmetry")
                    .requires("iterated_function_system")
              )
              .arg(Arg::with_name("vibrancy")
                    .long("vibrancy")
                    .takes_value(true)
                    .help("sets the vibrancy of the colors (between [0, 1])")
                    .requires("iterated_function_system")
              )
              .arg(Arg::with_name("gamma")
                    .long("gamma")
                    .takes_value(true)
                    .help("sets the gamma correction of the colors")
                    .requires("iterated_function_system")
              )
              .arg(Arg::with_name("variation")
                    .long("variation")
                    .takes_value(true)
                    .help(&format!("the variation to use for fractal flames\nOne of:\n  {}",
                                  Variation::list().join("\n  "))
                    )
                    // FIXME what is the nice way?
                    .possible_values(&Variation::list().iter().map(|s| s.as_ref()).collect::<Vec<&str>>().as_slice())
                    .requires("iterated_function_system")
              )
              .arg(Arg::with_name("rpn")
                  .long("rpn")
                  .long("reverse-polish-notation")
                  .help("give a formula to visualize as a newton fractal in reverse polish
                        notation, must be enclosed in quotations")
                  .takes_value(true)
                  .requires("escape_time")
              )
              .arg(Arg::with_name("zoom")
                  .long("zoom")
                  .help("how deep to zoom into the complex plane for Mandelbrot. The visible square will have a side length of 1/zoom")
                  .takes_value(true)
                  .requires("mandelbrot")
              )
              .arg(Arg::with_name("center")
                  .long("center")
                  .help("where to look at on the complex plane for Mandelbrot. Format: `x,y`")
                  .takes_value(true)
                  .allow_hyphen_values(true)
                  .requires("mandelbrot")
              )
              .arg(Arg::with_name("iterations")
                    .short("N")
                    .long("iterations")
                    .takes_value(true)
                    .help("sets the number of iterations for the L-system")
                    .requires("lsystem")
              )
              .arg(Arg::with_name("start")
                    .long("start")
                    .takes_value(true)
                    .help("sets the starting state for the L-system")
                    .requires("lsystem")
                    .requires("rules")
              )
              .arg(Arg::with_name("rules")
                    .long("rules")
                    .takes_value(true)
                    .help("sets the substitution rules for the L-system")
                    .requires("lsystem")
                    .requires("start")
              )
              .arg(Arg::with_name("angle")
                    .long("angle")
                    .takes_value(true)
                    .help("sets the angle for the L-system")
                    .requires("lsystem")
              )
              .arg(Arg::with_name("qmaprule")
                    .long("qmaprule")
                    .takes_value(true)
                    .help("draw the quaratic map of this 12 letter rule")
                    .requires("qmap")
              )
              .arg(Arg::with_name("bound")
                    .long("bound")
                    .possible_values(&["strict", "loose", "zoom"])
                    .takes_value(true)
                    .help("specify how the image is cropped")

              )
              .get_matches();

    let tweet = matches.is_present("tweet");
    let quiet = matches.is_present("quiet");
    let optipng = !matches.is_present("no-optipng");
    let supersampling = matches.is_present("supersampling");
    let filename = matches.value_of("filename")
                          .and_then(|f| Some(f.to_string()))
                          .or_else(|| None);
    // test if style is valid
    let style = match matches.value_of("style")
    {
        Some(x) => Some(Style::from_string(x).expect(&format!("Invalid Style {}", x))),
        None => None
    };

    let bounds = match matches.value_of("bound") {
        Some("strict") => Some(BoundsTypes::StrictBounds),
        Some("loose") => Some(BoundsTypes::BoundsWithoutOutliers),
        Some("zoom") => Some(BoundsTypes::ZoomedBounds),
        None => None,
        _ => unreachable!()
    };

    let seed = matches.value_of("seed")
                      .and_then(|s| Some(s.parse::<usize>().expect("seed needs to be an integer")))
                      .or_else(|| None);

    let height = matches.value_of("height")
                        .and_then(|s| Some(s.parse::<u32>().expect("height needs to be an integer")))
                        .or_else(|| None);
    let width = matches.value_of("width")
                       .and_then(|s| Some(s.parse::<u32>().expect("width needs to be an integer")))
                       .or_else(|| None);

    let vibrancy = matches.value_of("vibrancy")
                       .and_then(|s| Some(s.parse::<f64>().expect("vibrancy needs to be a number")))
                       .or_else(|| None);
    let gamma = matches.value_of("gamma")
                       .and_then(|s| Some(s.parse::<f64>().expect("gamma needs to be a number")))
                       .or_else(|| None);

    let rpn = matches.value_of("rpn")
                     .and_then(|f| Some(f.to_string()))
                     .or_else(|| None);

    let zoom = matches.value_of("zoom")
                     .and_then(|s| Some(s.parse::<u64>().expect("zoom needs to be a number")))
                     .or_else(|| None);
    let center = matches.value_of("center")
                     .and_then(|s| {
                         let c: Vec<&str> = s.split(",").collect();
                         if c.len() != 2 {panic!("center needs the format `x,y`")}
                         let x = c[0].parse::<f64>().expect("center needs the format `x,y`");
                         let y = c[1].parse::<f64>().expect("center needs the format `x,y`");
                         Some((x, y))
                     })
                     .or_else(|| None);

    let iterations = matches.value_of("iterations")
                       .and_then(|s| Some(s.parse::<u32>().expect("iterations needs to be a unsigned integer")))
                       .or_else(|| None);
    let angle = matches.value_of("angle")
                       .and_then(|s| Some(s.parse::<f64>().expect("angle needs to be a number") / 180.*PI))
                       .or_else(|| None);

    let rules = match (matches.value_of("start"), matches.value_of("rules"))
    {
        (Some(s), Some(r)) => Some(Lrules::from_string(s, r)),
        _ => None
    };

    let qmaprule = matches.value_of("qmaprule").map(|x| x.to_owned());

    let fractal_type = if matches.is_present("newton") {
        FractalType::Newton
    } else if matches.is_present("julia") {
        FractalType::Julia
    } else if matches.is_present("mandelbrot") {
        FractalType::Mandelbrot
    } else if matches.is_present("dragon") {
        FractalType::HeighwayDragon
    } else if matches.is_present("fern") {
        FractalType::BarnsleyFern
    } else if matches.is_present("sierpinski") {
        FractalType::SierpinskiGasket
    } else if matches.is_present("sierpinski-pentagon") {
        FractalType::SierpinskiPentagon
    } else if matches.is_present("pythagorean") {
        FractalType::PythagoreanTree
    } else if matches.is_present("appolonian") {
        FractalType::AppolonianGasket
    } else if matches.is_present("mobius") {
        FractalType::MobiusFlame
    } else if matches.is_present("flame") {
        FractalType::FractalFlame
    } else if matches.is_present("json") {
        let filename = matches.value_of("json").expect("file need to be specified");
        let mut file = fs::File::open(filename).expect("can not open file");
        let mut json = String::new();
        file.read_to_string(&mut json).expect("can not read file");
        FractalType::LoadJson(json)
    } else if matches.is_present("kochcurve") {
        FractalType::KochCurve
    } else if matches.is_present("sierpinskiarrowhead") {
        FractalType::SierpinskiArrowhead
    } else if matches.is_present("hilbertcurve") {
        FractalType::HilbertCurve
    } else if matches.is_present("gospercurve") {
        FractalType::GosperCurve
    } else if matches.is_present("bush") {
        FractalType::Bush
    } else if matches.is_present("penrose") {
        FractalType::PenroseTiling
    } else if matches.is_present("pentigree") {
        FractalType::Pentigree
    } else if matches.is_present("tritile") {
        FractalType::Tritile
    } else if matches.is_present("ldragon") {
        FractalType::LDragon
    } else if matches.is_present("random_lsystem") {
        FractalType::RandomLSystem
    } else if matches.is_present("ising") {
        FractalType::Ising
    } else if matches.is_present("quadraticmap") {
        FractalType::QuadraticMap
    } else if matches.is_present("lorenz") {
        FractalType::Lorenz
    } else if matches.is_present("rossler") {
        FractalType::Rossler
    } else if matches.is_present("thomas") {
        FractalType::Thomas
    } else if matches.is_present("doublependulum") {
        FractalType::DoublePendulum
    } else {
        FractalType::Random
    };

    let variation = matches.value_of("variation")
                           .and_then(|s| Variation::from_string_and_seed(s, seed));

    let symmetry = if matches.is_present("mirror") {
        Some(Symmetry::Vertical)
    } else if matches.is_present("mirror-horizontal") {
        Some(Symmetry::Horizontal)
    } else if matches.is_present("rotational") {
        let symmetries = matches.value_of("rotational")
                                .unwrap()
                                .parse::<usize>()
                                .expect("the number of rotational symmetries need to be an integer");
        Some(Symmetry::Rotational(symmetries))
    } else if matches.is_present("no-symmetry") {
        Some(Symmetry::None)
    } else {
        None
    };

    Options {
        seed,
        filename,
        style,
        tweet,
        quiet,
        fractal_type,
        height,
        width,
        bounds,
        supersampling,
        optipng,
        variation,
        symmetry,
        vibrancy,
        gamma,
        iterations,
        rules,
        angle,
        rpn,
        zoom,
        center,
        qmaprule,
    }
}
