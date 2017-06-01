#[derive(Debug, PartialEq)]
pub struct HSV(pub f64, pub f64, pub f64);

#[derive(Debug, PartialEq)]
pub struct RGB(pub f64, pub f64, pub f64);

impl HSV {
    pub fn to_rgb(&self) -> RGB {
        hsv2rgb(self)
    }
}

/// convert a hsv color representations into an rgb representation
fn hsv2rgb(hsv: &HSV) -> RGB {
    // https://de.wikipedia.org/wiki/HSV-Farbraum#Umrechnung_HSV_in_RGB
    let &HSV(h, s, v) = hsv;
    let hi = (h * 6.).floor() as u32;
    let f = h * 6. - hi as f64;
    let p = v*(1.-s);
    let q = v*(1.-s*f);
    let t = v*(1.-s*(1.-f));

    match hi {
        0 | 6 => RGB(v, t, p),
        1 => RGB(q, v, p),
        2 => RGB(p, v, t),
        3 => RGB(p, q, v),
        4 => RGB(t, p, v),
        5 => RGB(v, p, q),
        _ => RGB(0., 0., 0.)
    }
}


/// color_variance is an ad-hoc measure for the interestingness of an image
pub fn color_variance(pixels: &[HSV]) -> f64 {
    let n = pixels.len() as f64;
    let mean_h = 1./n * pixels.iter()
                              .map(|&HSV(h, _, _)| h)
                              .sum::<f64>();
    let mean_s = 1./n * pixels.iter()
                              .map(|&HSV(_, s, _)| s)
                              .sum::<f64>();
    let mean_v = 1./n * pixels.iter()
                              .map(|&HSV(_, _, v)| v)
                              .sum::<f64>();

    let var_h = 1./n * pixels.iter()
                             .map(|&HSV(h, _, _)| (h-mean_h) * (h-mean_h))
                             .sum::<f64>();
    let var_s = 1./n * pixels.iter()
                             .map(|&HSV(_, s, _)| (s-mean_s) * (s-mean_s))
                             .sum::<f64>();
    let var_v = 1./n * pixels.iter()
                             .map(|&HSV(_, _, v)| (v-mean_v) * (v-mean_v))
                             .sum::<f64>();

    // TODO: also avoid images where more than half pixels are black
    let tmp = (var_h, if var_s > var_v {var_s} else {var_v});
    (tmp.0 + tmp.1) / 2.
}
