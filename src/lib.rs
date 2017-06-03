#[macro_use] extern crate itertools;
#[macro_use] extern crate log;

pub mod iterated_fractal;
pub mod color;
pub mod numbers;
pub mod functions;

use std::process::Command;

use std::fmt;
use std::fs;

#[derive(Debug, Clone)]
pub enum FractalType {
    Random,
    Newton,
    Julia,
    Mandelbrot
}

impl fmt::Display for FractalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn postprocess_image(filename: &str) {
    // use optipng to compress the png further
    info!("compress with optipng");
    let before = fs::metadata(filename).map(|x| x.len()).unwrap_or(0);

    let output = Command::new("optipng")
                         .arg("-o7")
                         .arg(filename)
                         .output();

    let after = fs::metadata(filename).map(|x| x.len()).unwrap_or(0);

    match output {
        Ok(x) => if !x.status.success() {
                        error!("optipng failed")
                    } else {

                        info!("optipng successful ({} KiB -> {} KiB), saved {:.0}%",
                              before as f32 / 1000.,
                              after as f32 / 1000.,
                              1. - after as f32 / before as f32 * 100.)
                    },
        Err(x) => error!("optipng failed with {:?}", x)
    };
}

pub fn postprocess_image_for_twitter(input: &str, output: &str) {
    // since twitter will convert the pictures to jpg with artifacts,
    // add a transparent border to suppress the conversion
    // using imagemagick's convert
    let output = Command::new("convert")
                         .arg("-alpha").arg("on")
                         .arg("-channel").arg("RGBA")
                         .arg("-bordercolor").arg("rgba(0,0,0,0)")
                         .arg("-border").arg("1x1")
                         .arg(input)
                         .arg(output)
                         .output();

    match output {
        Ok(x) => if !x.status.success() {
                        error!("convert failed")
                    } else {
                        info!("convert successful")
                    },
        Err(x) => error!("convert failed with {:?}", x)
    };
}
