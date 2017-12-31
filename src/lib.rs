#[macro_use] extern crate itertools;
#[macro_use] extern crate log;

pub mod escape_time_fractal;
pub mod iterated_function_system;
pub mod color;
pub mod numbers;
pub mod functions;
pub mod histogram;
pub mod png;

use std::process::Command;

use std::fmt;
use std::fs;

/// Supported types of fractals
#[derive(Debug, Clone)]
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
    ThreeMobius,
    MobiusFlame,
    FractalFlame,
    MirrorFlame,
    SymmetricFlame,
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

/// add a transparent border to a png
///
/// # Arguments
///
/// * `input` - path to the source png
/// * `output` - path where the postprocessed png should be saved
///
/// # Remarks
///
/// Since twitter will convert uploaded png pictures to jpg with artifacts,
/// we add a transparent border to suppress the conversion
/// using imagemagick's convert.
///
/// Also, we need to ensure that the filesize is <= 3 MiB <https://dev.twitter.com/rest/media/uploading-media#imagerecs>.
/// therefore, we will shrink by 10%, measure the size and repeat, until we reach the limit.
///
/// *Note*: If imagemagick's `convert` is not in the path, this function
/// will do nothing, but logging an error.
pub fn postprocess_image_for_twitter(input: &str, outfile: &str) {
    let mut size: u64;
    let mut resize: f64 = 1.;

    let output = Command::new("convert")
                         .arg("-alpha").arg("on")
                         .arg("-channel").arg("RGBA")
                         .arg("-bordercolor").arg("rgba(0,0,0,0)")
                         .arg("-border").arg("1x1")
                         .arg(input)
                         .arg(outfile)
                         .output();

    size = fs::metadata(outfile).map(|x| x.len()).unwrap_or(0);

    match output {
        Ok(x) => if !x.status.success() {
                        error!("convert failed")
                    } else {
                        info!("convert successful ({} KiB)", size as f32 / 1000.)
                    },
        Err(x) => error!("convert failed with {:?}", x)
    };

    // it is too big, shrink it and try again
    while size > 3e6f64 as u64 {
        resize *= 0.9;
        info!("rescale to {:.0}%", resize*100.);

        let output = Command::new("convert")
                             .arg("-alpha").arg("on")
                             .arg("-channel").arg("RGBA")
                             .arg("-bordercolor").arg("rgba(0,0,0,0)")
                             .arg("-border").arg("1x1")
                             .arg("-scale").arg(format!("{:.0}%", resize*100.))
                             .arg(input)
                             .arg(outfile)
                             .output();

        size = fs::metadata(outfile).map(|x| x.len()).unwrap_or(0);

        match output {
            Ok(x) => if !x.status.success() {
                            error!("convert failed")
                        } else {
                            info!("convert successful ({} KiB)", size as f32 / 1000.)
                        },
            Err(x) => error!("convert failed with {:?}", x)
        };
    }
}
