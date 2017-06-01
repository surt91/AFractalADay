#[macro_use] extern crate itertools;
#[macro_use] extern crate log;

pub mod iterated_fractal;
pub mod color;
pub mod numbers;
pub mod functions;

use std::process::Command;

use std::fmt;

#[derive(Debug)]
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
                        error!("convert failed")
                    } else {
                        info!("convert successful")
                    },
        Err(x) => error!("failed with {:?}", x)
    };
}
