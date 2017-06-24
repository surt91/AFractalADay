extern crate clap;

use std::fmt;
use self::clap::{App, Arg, ArgGroup};
use iterated_fractal::style::Style;
use FractalType;

#[derive(Debug)]
pub struct Options {
    pub seed: Option<usize>,
    pub filename: Option<String>,
    pub style: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
    pub tweet: bool,
    pub quiet: bool,
    pub fractal_type: FractalType,
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Options: type: {}, seed: {}, name:  {}, style: {}, tweet: {}, quiet: {}",
                  self.fractal_type,
                  self.seed.map_or("random".to_string(), |s| s.to_string()),
                  self.filename.as_ref().unwrap_or(&"random".to_string()),
                  self.style.as_ref().unwrap_or(&"random".to_string()),
                  self.tweet,
                  self.quiet
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
              .arg(Arg::with_name("newton")
                    .long("newton")
                    .help("render a newton fractal")
              )
              .arg(Arg::with_name("julia")
                    .long("julia")
                    .help("render a julia fractal")
              )
              .arg(Arg::with_name("mandelbrot")
                    .long("mandelbrot")
                    .help("render a mandelbrot fractal")
              )
              .arg(Arg::with_name("dragon")
                    .long("dragon")
                    .help("render a dragon curve fractal")
              )
              .arg(Arg::with_name("fern")
                    .long("fern")
                    .help("render a Barnsley fern fractal")
              )
              .arg(Arg::with_name("flame")
                    .long("flame")
                    .help("render a fractal flame")
              )
              .group(ArgGroup::with_name("fractal_type")
                  .args(&["newton", "mandelbrot", "julia", "dragon", "fern", "flame"]))
              .get_matches();

    let tweet = matches.is_present("tweet");
    let quiet = matches.is_present("quiet");
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
                      .and_then(|s| Some(s.parse::<usize>().expect("seed needs to be and integer")))
                      .or_else(|| None);

    let height = matches.value_of("height")
                        .and_then(|s| Some(s.parse::<u32>().expect("height needs to be and integer")))
                        .or_else(|| None);;
    let width = matches.value_of("width")
                       .and_then(|s| Some(s.parse::<u32>().expect("width needs to be and integer")))
                       .or_else(|| None);;

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
    } else if matches.is_present("flame") {
        FractalType::FractalFlame
    } else {
        FractalType::Random
    };

    Options {
        seed,
        filename,
        style,
        tweet,
        quiet,
        fractal_type,
        height,
        width
    }
}
