use log::{info, error};

use serde::{Serialize, Deserialize};

pub mod fractal;
pub mod color;
pub mod colormap;
pub mod numbers;
pub mod histogram;
pub mod png_helper;

use std::process::Command;

use std::fmt;
use std::fs;

/// Supported types of fractals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FractalType {
    Random,
    Newton,
    Julia,
    Mandelbrot,
    HeighwayDragon,
    BarnsleyFern,
    SierpinskiGasket,
    SierpinskiPentagon,
    PythagoreanTree,
    AppolonianGasket,
    MobiusFlame,
    FractalFlame,
    LoadJson(String),
    KochCurve,
    SierpinskiArrowhead,
    HilbertCurve,
    GosperCurve,
    Bush,
    PenroseTiling,
    Pentigree,
    Tritile,
    LDragon,
    RandomLSystem,
    Ising,
    QuadraticMap,
    Lorenz,
    Rossler,
    Thomas,
    DoublePendulum,
}

impl fmt::Display for FractalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// compress a png  in place using `optipng`
///
/// # Arguments
///
/// * `filename` - filename of the png which should be compressed
///
/// # Remarks
///
/// The png crate will generate non-optimal png. By `optipng` the png
/// can typically be compressed by 30 - 50%.
///
/// *Note*: If `optipng` is not in the path, this function
/// will do nothing, but logging an error.
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

                        info!("optipng successful ({} KB -> {} KB), saved {:.0}%",
                              before as f32 / 1000.,
                              after as f32 / 1000.,
                              (1. - after as f32 / before as f32) * 100.)
                    },
        Err(x) => error!("optipng failed with {:?}", x)
    };
}

/// reduce the size to something twitter allows
///
/// # Arguments
///
/// * `input` - path to the source png
/// * `output` - path where the postprocessed png should be saved
///
/// # Remarks
///
/// We need to ensure that the filesize is <= 5 MiB.
/// therefore, we will shrink by 10%, measure the size and repeat, until we reach the limit.
///
/// *Note*: If imagemagick's `convert` is not in the path, this function
/// will do nothing, but logging an error.
pub fn postprocess_image_for_twitter(input: &str, outfile: &str) {
    let mut size: u64;
    let mut resize: f64 = 1.;

    fs::copy(input, outfile).expect("input image does not exist!");
    postprocess_image(outfile);

    size = fs::metadata(outfile).map(|x| x.len()).unwrap_or(0);
    // if it is too big, shrink it and try again
    // the threshold is 5 MB
    while size > 4.9e6f64 as u64 {
        resize *= 0.9;
        info!("rescale to {:.0}%", resize*100.);

        let output = Command::new("convert")
                             .arg("-scale").arg(format!("{:.0}%", resize*100.))
                             .arg(input)
                             .arg(outfile)
                             .output();
        postprocess_image(outfile);

        size = fs::metadata(outfile).map(|x| x.len()).unwrap_or(0);

        match output {
            Ok(x) => if !x.status.success() {
                            error!("convert failed")
                        } else {
                            info!("convert successful ({} KB)", size as f32 / 1000.)
                        },
            Err(x) => error!("convert failed with {:?}", x)
        };
    }
}
