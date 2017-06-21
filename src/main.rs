extern crate a_fractal_a_day;

use a_fractal_a_day::*;
use iterated_fractal::IteratedFractal;
use iterated_fractal::iterated_fractal_builder::IteratedFractalBuilder;
use iterated_function_system::IteratedFunctionSystem;
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
use iterated_fractal::style::Style;



// only log errors to stdout, but everything to a log file
fn init_logging(quiet: bool) {
    let level = if quiet {LogLevelFilter::Error} else {LogLevelFilter::Info};
    log_panics::init();
    let _ = CombinedLogger::init(
        vec![
            SimpleLogger::new(level, Config::default()),
            WriteLogger::new(LogLevelFilter::Info, Config::default(),
                             fs::OpenOptions::new().append(true)
                                                   .create(true)
                                                   .open("fractals.log")
                                                   .expect("Failed to create log file!"))
        ]
    );
}


fn prepare(filename: &str) -> String {
    fs::create_dir_all("img").expect("could not create output directory");

    format!("img/{}.png", filename)
}

fn render_fractal<T: IteratedFractal>(fractal: &mut T, filename: &str, dim: &(u32, u32)) -> (bool, String) {
    let mut finished = false;
    // ensure that the image has some variance
    // otherwise the images are probably boring
    let &(w, h) = dim;
    match fractal.render((w, h), None, None, filename) {
        Ok(variance) => finished = variance > 0.01,
        Err(x) => error!("creation of fractal failed {:?}", x)
    }

    let description = fractal.description().to_string();
    info!("{}", description);

    (finished, description)
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
            0 => FractalType::Julia,
            1 => FractalType::Mandelbrot,
            2 => FractalType::Newton,
            _ => unreachable!()
        }
    };

    let dim = (opt.width.unwrap_or(2046), opt.height.unwrap_or(1022));

    // hacky do while loop
    while {
        let mut a = IteratedFractalBuilder::new().seed(seed+ctr);
        a = match opt.style {
            // the parser made sure that this is a valid value, unwrap should be fine
            Some(ref x) => a.style(Style::from_string(x).unwrap()),
            None => a
        };
        let mut b = IteratedFunctionSystemBuilder::new().seed(seed+ctr);
        b = b.iterations((dim.0 * dim.1 * 100) as usize);
        // b = match opt.style {
        //     // the parser made sure that this is a valid value, unwrap should be fine
        //     Some(ref x) => b.style(Style::from_string(x).unwrap()),
        //     None => b
        // };

        let (finished, tmp_description) = match fractal_type {
            FractalType::Newton => render_fractal(&mut a.newton(), filename, &dim),
            FractalType::Julia => render_fractal(&mut a.julia(), filename, &dim),
            FractalType::Mandelbrot => render_fractal(&mut a.mandelbrot(), filename, &dim),
            FractalType::HeighwayDragon => {
                let mut f = false;
                let mut fractal = b.heighway_dragon();
                match fractal.render(dim, None, None, filename) {
                    Ok(variance) => f = variance > 0.01,
                    Err(x) => error!("creation of fractal failed {:?}", x)
                }

                let description = fractal.description().to_string();
                info!("{}", description);

                (f, description)
            },
            FractalType::Random => unreachable!()
        };

        description = tmp_description;
        ctr += 1;
        ! finished
    } {}

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
    let filename = opt.filename.clone().unwrap_or_else(|| timestamp.to_string());
    let filename = prepare(&filename);

    info!("start generation with seed {}", seed);

    let description = build_fractal(&filename, seed, &opt);

    info!("image saved as {}", filename);

    if opt.tweet {
        let for_twitter = format!("{}_for_twitter.png", filename);
        postprocess_image_for_twitter(&filename, &for_twitter);
        info!("start upload to twitter");
        tweet(&for_twitter, &description);
        info!("tweeted");
        fs::remove_file(&for_twitter).unwrap_or_else(|_| warn!("could not delete {}", for_twitter));
    }

    postprocess_image(&filename);

    info!("Success!");
}
