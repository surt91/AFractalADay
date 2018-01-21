//! Generate random fractals.

extern crate a_fractal_a_day;

use a_fractal_a_day::*;
use fractal::{FractalBuilder,render_wrapper};

use std::fs;
use std::io::prelude::*;

extern crate rand;
use self::rand::{Rng, StdRng};

#[macro_use] extern crate log;
extern crate log_panics;
extern crate simplelog;
use simplelog::{ CombinedLogger, SimpleLogger, WriteLogger, LogLevelFilter, Config};

extern crate time;

extern crate my_twitter;
use my_twitter::twitter as twitter;

mod parse_cl;
use parse_cl::{parse_cl, Options};


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


fn prepare(filename: &str) -> (String, String) {
    fs::create_dir_all("img").expect("could not create output directory");
    fs::create_dir_all("json").expect("could not create output directory");

    let img = format!("img/{}.png", filename);
    let json = format!("json/{}.json", filename);

    (img, json)
}

fn build_fractal(filename: &str,
                  seed: usize,
                  opt: &Options) -> (String, String) {
    let mut description;
    let mut json;
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
        let mut fractal = FractalBuilder::new()
                                         .seed(seed+ctr)
                                         .style(&opt.style)
                                         .variation(&opt.variation)
                                         .symmetry(&opt.symmetry)
                                         .build(&fractal_type);

        let (finished, tmp_description, tmp_json) = render_wrapper(&mut fractal, filename, &dim);

        description = tmp_description;
        json = tmp_json;
        ctr += 1;
        ! finished
    } {}

    (description, json)
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
    let (file_img, file_json) = prepare(&filename);

    info!("start generation with seed {}", seed);

    let (description, json) = build_fractal(&file_img, seed, &opt);
    let mut file = fs::File::create(file_json).unwrap();
    file.write_all(json.as_bytes()).unwrap();

    info!("image saved as {}", file_img);

    if opt.optipng {
        postprocess_image(&file_img);
    }

    if opt.tweet {
        let for_twitter = format!("{}_for_twitter.png", file_img);
        postprocess_image_for_twitter(&file_img, &for_twitter);
        info!("start upload to twitter");
        tweet(&for_twitter, &description);
        info!("tweeted");
        fs::remove_file(&for_twitter).unwrap_or_else(|_| warn!("could not delete {}", for_twitter));
    }

    info!("Success!");
}
