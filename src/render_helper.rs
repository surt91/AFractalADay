extern crate serde_json;

use fractal::{Fractal};

pub fn render_wrapper(fractal: &mut Fractal, filename: &str, dim: &(u32, u32)) -> (bool, String, String) {
    // for some fractals, we can estimate if it will look good
    // so abort, if not before rendering
    if ! fractal.estimate_quality() {
        return (false, "".to_string(), "".to_string())
    }

    let variance = fractal.render(*dim, filename).expect("creation of fractal failed");

    let description = fractal.description().to_owned();
    info!("{}", description);

    let json = serde_json::to_string_pretty(&fractal.json()).unwrap();

    // ensure that the image has some variance
    // otherwise the images are probably boring
    let finished = variance > 0.01;
    // TODO: we need something better than the variance to estimate the
    // quality of an image, maybe do an FFT and look for intermediate frequencies?

    (finished, description, json)
}
