extern crate a_fractal_a_day;

use a_fractal_a_day::*;
use iterated_fractal::IteratedFractal;
use iterated_fractal::iterated_fractal_builder::IteratedFractalBuilder;

use std::fs;

#[macro_use]
extern crate log;
extern crate log_panics;
extern crate simplelog;
use simplelog::{ CombinedLogger, SimpleLogger, WriteLogger, LogLevelFilter, Config};

extern crate time;

extern crate my_twitter;
use my_twitter::twitter as twitter;

mod parse_cl;
use parse_cl::parse_cl;


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
        let mut a = IteratedFractalBuilder::new().seed(seed+ctr)
                                                 .newton();

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
