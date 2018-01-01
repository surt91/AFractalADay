//! Generate random fractals.

extern crate a_fractal_a_day;

use a_fractal_a_day::*;
use escape_time_fractal::escape_time_fractal_builder::EscapeTimeFractalBuilder;
use iterated_function_system::iterated_function_system_builder::IteratedFunctionSystemBuilder;

use std::fs;

extern crate rand;
use self::rand::Rng;
use self::rand::StdRng;

#[macro_use] extern crate log;
extern crate log_panics;
extern crate simplelog;
use simplelog::{ CombinedLogger, SimpleLogger, WriteLogger, LogLevelFilter, Config};

extern crate time;

extern crate my_twitter;
use my_twitter::twitter as twitter;

mod parse_cl;
use parse_cl::{parse_cl, Options};
use escape_time_fractal::style::Style;

mod render_helper;
use render_helper::{render_escape_time_fractal, render_ifs, render_fractal_flame};


// only log errors to stdout, but everything to a log file
fn init_logging(quiet: bool) {
    let level = if quiet {LogLevelFilter::Error} else {LogLevelFilter::Info};
    log_panics::init();
    let _ = CombinedLogger::init(
        vec![
            SimpleLogger::new(level, Config::default()),
            WriteLogger::new(level, Config::default(),
                             fs::OpenOptions::new().append(true)
                                                   .create(true)
                                                   .open("fractals.log")
                                                   .expect("Failed to create log file!")),
            WriteLogger::new(LogLevelFilter::Debug, Config::default(),
                             fs::OpenOptions::new().append(true)
                                                   .create(true)
                                                   .open("fractals.debug")
                                                   .expect("Failed to create log file!"))
        ]
    );
}


fn prepare(filename: &str) -> String {
    fs::create_dir_all("img").expect("could not create output directory");

    format!("img/{}.png", filename)
}

fn build_fractal(filename: &str,
                  seed: usize,
                  opt: &Options) -> String{
    let mut description;
    let mut ctr = 0;
    let mut fractal_type: FractalType = opt.fractal_type.clone();

    let tmp: &[_] = &[seed];
    let mut rng: StdRng = rand::SeedableRng::from_seed(tmp);

    if let FractalType::Random = fractal_type {
        fractal_type = match rng.gen_range(0, 3) {
            0 => FractalType::Newton,
            1 => FractalType::FractalFlame,
            2 => FractalType::MobiusFlame,
            _ => unreachable!()
        }
    };

    let dim = (opt.width.unwrap_or(2046), opt.height.unwrap_or(1022));

    // hacky do while loop
    while {
        let mut a = EscapeTimeFractalBuilder::new().seed(seed+ctr);
        a = match opt.style {
            // the parser made sure that this is a valid value, unwrap should be fine
            Some(ref x) => a.style(Style::from_string(x).unwrap()),
            None => a
        };
        let s = seed+ctr;
        let mut b = IteratedFunctionSystemBuilder::new().seed(s);
        b = match opt.variation {
            Some(ref x) => b.variation(x),
            None => b
        };
        b = match opt.symmetry {
            Some(ref x) => b.symmetry(x),
            None => b
        };

        let (finished, tmp_description) = match fractal_type {
            FractalType::Newton => render_escape_time_fractal(&mut a.newton(), filename, &dim),
            FractalType::Julia => render_escape_time_fractal(&mut a.julia(), filename, &dim),
            FractalType::Mandelbrot => render_escape_time_fractal(&mut a.mandelbrot(), filename, &dim),
            FractalType::HeighwayDragon => render_ifs(&mut b.heighway_dragon(), filename, &dim),
            FractalType::BarnsleyFern => render_ifs(&mut b.barnsley_fern(), filename, &dim),
            FractalType::SierpinskiGasket => render_ifs(&mut b.sierpinski_gasket(), filename, &dim),
            FractalType::SierpinskiPentagon => render_ifs(&mut b.sierpinski_pentagon(), filename, &dim),
            FractalType::PythagoreanTree => render_fractal_flame(&mut b.pythagorean_tree(), filename, &dim),
            FractalType::MobiusFlame => render_fractal_flame(&mut b.mobius_flame(), filename, &dim),
            FractalType::FractalFlame => render_fractal_flame(&mut b.fractal_flame(), filename, &dim),
            FractalType::Random => unreachable!()
        };

        description = tmp_description;
        ctr += 1;
        ! finished
    } {}

    description
}

fn tweet(filename: &str, description: &str) {
    use std::thread::sleep;
    use std::time;
    twitter::tweet_image(description, filename)
            .or_else(|_| {
                    info!("Upload to Twitter failed!");
                    info!("Try again in 60 seconds.");
                    sleep(time::Duration::from_secs(60));
                    twitter::tweet_image(description, filename)
                }
            )
            .or_else(|_| {
                    info!("Upload to Twitter failed again!");
                    info!("Try again in 10 minutes.");
                    sleep(time::Duration::from_secs(600));
                    twitter::tweet_image(description, filename)
                }
            )
            .expect("Uploading to twitter failed three times! Panic!");
}

fn main() {
    let timestamp = time::now_utc().to_timespec().sec;

    let opt = parse_cl();
    init_logging(opt.quiet);
    info!("Start!");
    info!("{}", opt);

    let seed = opt.seed.unwrap_or(timestamp as usize);
    let filename = opt.filename.clone().unwrap_or_else(|| timestamp.to_string());
    let filename = prepare(&filename);

    info!("start generation with seed {}", seed);

    let description = build_fractal(&filename, seed, &opt);

    info!("image saved as {}", filename);

    if opt.optipng {
        postprocess_image(&filename);
    }

    if opt.tweet {
        let for_twitter = format!("{}_for_twitter.png", filename);
        postprocess_image_for_twitter(&filename, &for_twitter);
        info!("start upload to twitter");
        tweet(&for_twitter, &description);
        info!("tweeted");
        fs::remove_file(&for_twitter).unwrap_or_else(|_| warn!("could not delete {}", for_twitter));
    }

    info!("Success!");
}
