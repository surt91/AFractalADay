use numbers::Real;

use rand::{self, Rng};
use color::RGBA;


/// Estimates if the resulting fractal will be interesting
///
/// # Arguments
///
/// * `vals` - slice of a sample of points visited by the IFS
/// * `bounds` - approximate bounds of the attractor
///
/// # Remarks
///
/// Uses the techniques from <http://sprott.physics.wisc.edu/pubs/paper210.pdf>
/// and some ad hoc heuristics.
pub fn probably_good(vals: &[[Real; 2]], bounds: (f32, f32, f32, f32)) -> bool {
    let width = bounds.1 - bounds.0;
    let height = bounds.3 - bounds.2;

    // if we have NaN, it can not be good
    if !width.is_finite() || !height.is_finite() {
        info!("non-finite bounds, probably divergent");
        return false
    }

    // if it is too tall, it can not be good
    if width < height/2. {
        info!("probably too tall ({}:{:.1})", 1., height/width);
        return false
    }

    let dim = correlation_dimension(vals, width+height);
    if dim < 1.2 || dim.is_nan() {
        info!("bad correlation dimension ({})", dim);
        return false
    }
    info!("correlation dimension: {}", dim);

    // TODO: llyapunov exponent

    true
}

/// Calculates the correlation dimension.
///
/// # Arguments
///
/// * `vals` - slice of a sample of points visited by the IFS
/// * `span` - total size of the attractor
///
/// # Remarks
///
/// The correlation dimension is an estimate for the fractal dimension.
/// Fractals with a dimension larger than one are subjectively more pleasing.
/// See also <http://sprott.physics.wisc.edu/pubs/paper210.pdf>
fn correlation_dimension(vals: &[[Real; 2]], span: Real) -> Real {
    let mut n1: Real = 0.;
    let mut n2: Real = 0.;
    let r1: Real = span/100.;
    let r2: Real = 10.*r1;

    let mut rng = rand::weak_rng();

    for (n, i) in vals.iter().enumerate().skip(20) {
        let j = vals[rng.gen_range(0, n)];
        let r = ((i[0] - j[0]).powi(2) + (i[1] - j[1]).powi(2)).sqrt();
        if r < r1 {
            n1 += 1.;
        }
        if r < r2 {
            n2 += 1.;
        }
    }

    (n2/n1).ln() / (r2/r1).ln()
}

/// Calculates the entropy of an image.
///
/// # Arguments
///
/// * `pixel` - slice of the pixels of the image
///
/// # Remarks
///
/// Low entropy is a image of one color, high entropy is noise.
/// Interesting images should be located at some range of intermediate entropies.
pub fn entropy(pixel: &[RGBA]) -> f64 {
    use histogram::histogram1d;
    let hist = histogram1d(pixel.iter().map(|c| c.to_u8() as usize), (0, 255));

    let entropy = - hist.iter().map(|&i| {
                    let f = i as f64;
                    if f<=0. { 0. } else { f*f.log2() }
                }).sum::<f64>();

    entropy
}
