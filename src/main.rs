extern crate a_fractal_a_day;

use a_fractal_a_day::*;
use newton_fractal::NewtonFractalBuilder;

use std::fs;
use std::fmt;

#[macro_use]
extern crate log;
extern crate log_panics;
extern crate simplelog;
use simplelog::{ CombinedLogger, SimpleLogger, WriteLogger, LogLevelFilter, Config};

extern crate time;

extern crate clap;
use clap::{App, Arg};

extern crate my_twitter;
use my_twitter::twitter as twitter;

#[derive(Debug)]
struct Options {
    seed: Option<usize>,
    filename: Option<String>,
    tweet: bool,
    quiet: bool
}

impl fmt::Display for Options {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Options: seed: {}, name:  {}, tweet: {}, quiet: {}",
                  self.seed.map_or("random".to_string(), |s| s.to_string()),
                  self.filename.as_ref().unwrap_or(&"random".to_string()),
                  self.tweet,
                  self.quiet
              )
    }
}

// only log errors to stdout, but everything to a log file
fn init_logging(quiet: bool) {
    let level = if quiet {LogLevelFilter::Error} else {LogLevelFilter::Info};
    log_panics::init();
    let _ = CombinedLogger::init(
        vec![
            SimpleLogger::new(level, Config::default()),
            WriteLogger::new(LogLevelFilter::Info, Config::default(),
                             fs::OpenOptions::new().append(true)
                                                   .open("fractals.log")
                                                   .expect("Failed to create log file!"))
        ]
    );
}

fn parse_cl() -> Options {
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
                    .short("s")
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
              .arg(Arg::with_name("quiet")
                    .short("q")
                    .long("quiet")
                    .help("do only print error messages")
              )
              .get_matches();

    let tweet = matches.is_present("tweet");
    let quiet = matches.is_present("quiet");
    let filename = matches.value_of("filename")
                          .and_then(|f| Some(f.to_string()))
                          .or_else(|| None);
    let seed = matches.value_of("seed")
                      .and_then(|s| Some(s.parse::<usize>().expect("seed needs to be and integer")))
                      .or_else(|| None);

    Options {seed: seed, filename: filename, tweet: tweet, quiet: quiet}
}

fn prepare(filename: &str) -> String {
    fs::create_dir_all("img").expect("could not create output directory");

    format!("img/{}.png", filename)
}

fn render_fractal(filename: &str, seed: usize) -> String{
    let mut finished = false;

    let mut description;
    let mut ctr = 0;
    // hacky do while loop
    while {
        let mut a = NewtonFractalBuilder::new().seed(seed+ctr)
                                               .build();

        // ensure that the image has some variance
        // otherwise the images are probably boring
        match a.render((2048-2, 1024-2), filename) {
            Ok(variance) => finished = variance > 0.01,
            Err(x) => error!("creation of fractal failed {:?}", x)
        }

        info!("{}", a.description);
        description = a.description;

        ctr += 1;
        ! finished
    } {}

    postprocess_image(filename);

    description
}

fn tweet(filename: &str, description: &str) {
    twitter::tweet_image(description, filename)
            .expect("Uploading to twitter failed!");
}

fn main() {
    let timestamp = time::now_utc().to_timespec().sec;

    let opt = parse_cl();
    init_logging(opt.quiet);
    info!("Start!");
    info!("{}", opt);

    let seed = opt.seed.unwrap_or(timestamp as usize);
    let filename = opt.filename.unwrap_or_else(|| timestamp.to_string());
    let filename = prepare(&filename);

    info!("start generation with seed {}", seed);

    let description = render_fractal(&filename, seed);

    info!("image saved as {}", filename);

    if opt.tweet {
        info!("start upload to twitter");
        tweet(&filename, &description);
        info!("tweeted");
    }
    info!("Success!");
}
