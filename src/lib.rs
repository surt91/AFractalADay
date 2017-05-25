#![feature(closure_to_fn_coercion)]

#[macro_use] extern crate itertools;

pub mod newton_fractal;
pub mod twitter;

use std::process::Command;

pub fn postprocess_image(filename: &str) {
    // since twitter will convert the pictures to jpg with artifacts,
    // add a transparent border to suppress the conversion
    // using imagemagick's convert
    let output = Command::new("convert")
                         .arg("-alpha").arg("on")
                         .arg("-channel").arg("RGBA")
                         .arg("-bordercolor").arg("rgba(0,0,0,0)")
                         .arg("-border").arg("1x1")
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
