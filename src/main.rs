extern crate a_fractal_a_day;

use a_fractal_a_day::*;
use newton_fractal::NewtonFractal;

use std::fs;

extern crate time;

fn main() {
    let mut finished = false;
    let mut detail = String::new();
    fs::create_dir_all("img").expect("could not create output directory");
    let timestamp = time::now_utc().to_timespec().sec;
    let output = format!("img/{}.png", timestamp);

    let mut a;
    let mut ctr = 0;
    // hacky do while loop
    while {
        a = NewtonFractal::new(None, Some(&[timestamp as usize + ctr]));
        println!("{}", a.formula);

        // ensure that we do at least 10 million iterations
        // otherwise the images are probably boring
        match a.render((2048-2, 1024-2), &output) {
            Ok(x) => finished = x > 10000000,
            Err(x) => println!("creation of fractal failed {:?}", x)
        }

        ctr += 1;
        ! finished
    } {}

    postprocess_image(&output);
    twitter::tweet_image(&a.formula, &output).expect("Uploading to twitter failed!");
}
