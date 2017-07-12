extern crate clap;

use std::fmt;
use self::clap::{App, Arg, ArgGroup};
use escape_time_fractal::style::Style;
use FractalType;
use iterated_function_system::variation::Variation;

#[derive(Debug)]
pub struct Options {
    pub seed: Option<usize>,
    pub filename: Option<String>,
    pub style: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub tweet: bool,
    pub quiet: bool,
    pub optipng: bool,
    pub fractal_type: FractalType,
    pub variation: Option<Variation>,
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Options: type: {}, seed: {}, name:  {}, style: {}, variation: {}, tweet: {}, quiet: {}, optipng: {}",
                  self.fractal_type,
                  self.seed.map_or("random".to_string(), |s| s.to_string()),
                  self.filename.as_ref().unwrap_or(&"random".to_string()),
                  self.style.as_ref().unwrap_or(&"random".to_string()),
                  self.variation.as_ref().and_then(|v| Some(v.name())).unwrap_or_else(|| "default".to_string()),
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
              .group(ArgGroup::with_name("iterated_function_system")
                  .conflicts_with("escape_time")
              )
              .group(ArgGroup::with_name("escape_time")
                  .conflicts_with("iterated_function_system")
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
              .get_matches();

    let tweet = matches.is_present("tweet");
    let quiet = matches.is_present("quiet");
    let optipng = !matches.is_present("no-optipng");
    let filename = matches.value_of("filename")
                          .and_then(|f| Some(f.to_string()))
                          .or_else(|| None);
    // test if style is valid
    match matches.value_of("style")
    {
        Some(x) => Some(Style::from_string(x).expect(&format!("Invalid Style {}", x))),
        None => None
    };
    let style = matches.value_of("style").map(|x| x.to_string());
    let seed = matches.value_of("seed")
                      .and_then(|s| Some(s.parse::<usize>().expect("seed needs to be an integer")))
                      .or_else(|| None);

    let height = matches.value_of("height")
                        .and_then(|s| Some(s.parse::<u32>().expect("height needs to be an integer")))
                        .or_else(|| None);
    let width = matches.value_of("width")
                       .and_then(|s| Some(s.parse::<u32>().expect("width needs to be an integer")))
                       .or_else(|| None);

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
    } else if matches.is_present("mobius") {
        FractalType::MobiusFlame
    } else if matches.is_present("flame") {
        FractalType::FractalFlame
    } else {
        FractalType::Random
    };

    let variation = matches.value_of("variation")
                           .and_then(|s| Variation::from_string(s));

    Options {
        seed,
        filename,
        style,
        tweet,
        quiet,
        fractal_type,
        height,
        width,
        optipng,
        variation,
    }
}
