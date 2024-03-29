use crate::color::RGBA;

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
pub fn _entropy(pixel: &[RGBA]) -> f64 {
    use crate::histogram::histogram1d;
    let hist = histogram1d(pixel.iter().map(|c| c.to_u8() as usize), (0, 255));

    let entropy = - hist.iter().map(|&i| {
                        let f = i as f64;
                        if f<=0. { 0. } else { f*f.log2() }
                    }).sum::<f64>();

    entropy
}

pub fn _downscale(pixel: &[RGBA], resolution: &(u32, u32)) -> Vec<RGBA> {
    let &(x, y) = resolution;
    let x: usize = x as usize / 2;
    let y: usize = y as usize / 2;

    let mut out = vec![RGBA(0,0,0,0); x*y];

    for i in 0..x {
        for j in 0..y {
            out[j*x + i] = RGBA::blend(&[
                                            &pixel[ 2*j    * 2*x +  2*i],
                                            &pixel[(2*j+1) * 2*x +  2*i],
                                            &pixel[ 2*j    * 2*x + (2*i+1)],
                                            &pixel[(2*j+1) * 2*x + (2*i+1)]
                                        ]);
        }
    }

    out
}
