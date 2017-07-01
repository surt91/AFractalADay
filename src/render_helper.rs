use escape_time_fractal::EscapeTimeFractal;
use iterated_function_system::IteratedFunctionSystem;

pub fn render_escape_time_fractal<T: EscapeTimeFractal>(fractal: &mut T, filename: &str, dim: &(u32, u32)) -> (bool, String) {
    let mut finished = false;
    // ensure that the image has some variance
    // otherwise the images are probably boring
    match fractal.render(*dim, None, None, filename) {
        Ok(variance) => finished = variance > 0.01,
        Err(x) => error!("creation of fractal failed {:?}", x)
    }

    let description = fractal.description().to_string();
    info!("{}", description);

    (finished, description)
}

pub fn render_fractal_flame<T: IteratedFunctionSystem>(fractal: &mut T, filename: &str, dim: &(u32, u32)) -> (bool, String) {
    let mut finished = false;

    let description = fractal.description().to_string();
    info!("{}", description);

    // if the fractal will probably be not interesting, try the next one
    if !fractal.estimate_quality() {
        return (false, description)
    }

    // ensure that the image has some variance
    // otherwise the images are probably boring
    match fractal.render(*dim, 1000, filename) {
        Ok(variance) => finished = variance > 0.01,
        Err(x) => error!("creation of fractal failed {:?}", x)
    }

    (finished, description)
}


// same as fractal flame, but always accept
pub fn render_ifs<T: IteratedFunctionSystem>(fractal: &mut T, filename: &str, dim: &(u32, u32)) -> (bool, String) {
    let (_, description) = render_fractal_flame(fractal, filename, dim);
    (true, description)
}
