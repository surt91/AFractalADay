#![feature(closure_to_fn_coercion)]

#[macro_use] extern crate itertools;
extern crate time;

use std::fs;

mod newton_fractal;
mod twitter;

use newton_fractal::NewtonFractal;
use std::process::Command;

fn postprocess_image(filename: &str) {
    // since twitter will convert the pictures to jpg with artifacts,
    // add a transparent border to suppress the conversion
    // using imagemagick's convert
    let output = Command::new("convert")
                         .arg("-alpha on")
                         .arg("-channel RGBA")
                         .arg("-bordercolor \"rgba(0,0,0,0)\"")
                         .arg("-border \"1x1\"")
                         .arg(filename)
                         .arg(filename)
                         .output();
    match output {
        Ok(x) => if !x.status.success() {
                        println!("convert failed")
                    } else {
                        println!("convert successful")
                    },
        Err(x) => println!("failed with {:?}", x)
    };
}

fn main() {
    let mut finished = false;
    let mut detail = String::new();
    fs::create_dir_all("img").expect("could not create output directory");
    let output = format!("img/{}.png", time::now_utc().to_timespec().sec);

    while ! finished {
        let f = NewtonFractal::random_formula();
        println!("{}", f.1);
        let a = NewtonFractal::new(f.0);

        detail = format!("{} {}", "Newton Fractal", f.1);

        // ensure that we do at least 10 million iterations
        // otherwise the images are probably boring
        match a.render((2048-2, 1024-2), &output) {
            Ok(x) => finished = x > 10000000,
            Err(x) => println!("creation of fractal failed {:?}", x)
        }
    }

    postprocess_image(&output);
    twitter::tweet_image(&detail, "test.png").expect("Uploading to twitter failed!");
}
