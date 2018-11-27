// TODO: longterm change this to (u8, u8, u8)

/// data structure representing a HSV color value
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct HSV(pub f64, pub f64, pub f64);

/// data structure representing a RGB color value
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RGB(pub f64, pub f64, pub f64);

/// data structure representing a RGBA color value
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RGBA(pub u8, pub u8, pub u8, pub u8);

impl HSV {
    /// convert `HSV` into `RGB`
    pub fn to_rgb(&self) -> RGB {
        hsv2rgb(self)
    }
    pub fn to_rgba(&self) -> RGBA {
        let RGB(r, g, b) = hsv2rgb(self);
        RGBA((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8, 255)
    }

    pub fn to_u8(&self) -> u8 {
        self.to_rgb().to_u8()
    }
}

impl RGB {
    /// convert `RGB` into `HSV`
    pub fn to_hsv(&self) -> HSV {
        rgb2hsv(self)
    }

    pub fn to_u8(&self) -> u8 {
        let &RGB(r, g, b) = self;
        ((8. * r) as u8) << 5 | ((8. * g) as u8) << 2 | ((4.*b) as u8)
    }

    pub fn interpolate(a: &RGB, b: &RGB) -> RGB {
        let &RGB(r1, g1, b1) = a;
        let &RGB(r2, g2, b2) = b;
        RGB((r1+r2)/2., (g1+g2)/2., (b1+b2)/2.)
    }

    pub fn interpolate_weight(a: &RGB, b: &RGB, p: f64) -> RGB {
        let &RGB(r1, g1, b1) = a;
        let &RGB(r2, g2, b2) = b;
        let q = 1.-p;
        RGB(p*r1+q*r2, p*g1+q*g2, p*b1+q*b2)
    }
}

impl RGBA {
    /// convert `RGBA` to `RGB` by blending with a black background
    pub fn blend_black(&self) -> RGB {
        let &RGBA(r, g, b, a) = self;
        let alpha = a as f64 / 255.;
        RGB(r as f64 / 255. * alpha,
            g as f64 / 255. * alpha,
            b as f64 / 255. * alpha)
    }

    /// convert `RGBA` to `RGB` by blending with a white background
    pub fn blend_white(&self) -> RGB {
        let &RGBA(r, g, b, a) = self;
        let alpha = a as f64 / 255.;
        RGB(r as f64 / 255. * alpha + (1. - alpha),
            g as f64 / 255. * alpha + (1. - alpha),
            b as f64 / 255. * alpha + (1. - alpha))
    }

    /// convert `RGBA` to `RGB` by discarding the alpha channel
    pub fn blend_discard(&self) -> RGB {
        let &RGBA(r, g, b, _) = self;
        RGB(r as f64 / 255.,
            g as f64 / 255.,
            b as f64 / 255.)
    }

    pub fn blend(colors: &[&RGBA]) -> RGBA {
        let mut r = 0.;
        let mut g = 0.;
        let mut b = 0.;
        let mut a = 0.;
        let mut ctr = 0.;
        for c in colors {
            let &&RGBA(rr, gg, bb, aa) = c;
            r += rr as f64;
            g += gg as f64;
            b += bb as f64;
            a += aa as f64;
            ctr += 1.0;
        }
        RGBA(
            (r/ctr) as u8,
            (g/ctr) as u8,
            (b/ctr) as u8,
            (a/ctr) as u8
        )
    }

    pub fn to_u8(&self) -> u8 {
        self.blend_black().to_u8()
    }
}

/// convert a hsv color representations into a rgb representation
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

/// convert a rgb color representations into a hsv representation
fn rgb2hsv(rgb: &RGB) -> HSV {
    // https://de.wikipedia.org/wiki/HSV-Farbraum#Umrechnung_HSV_in_RGB
    let &RGB(r, g, b) = rgb;
    let max = if r >= g && r >= b {r} else if g >= r && g >= b {g} else {b};
    let min = if r <= g && r <= b {r} else if g <= r && g <= b {g} else {b};

    let mut h = if max == min {
        0.
    } else if max == r {
        1./6. * (0. + (g-b)/(max-min))
    } else if max == g {
        1./6. * (2. + (b-r)/(max-min))
    } else if max == b {
        1./6. * (4. + (r-g)/(max-min))
    } else {
        unreachable!()
    };

    if h < 0. {
        h += 1.;
    }

    let s = if max == 0. {
        0.
    } else {
        (max - min) / max
    };

    let v = max;

    HSV(h, s, v)
}

/// count the number of `pixels` with the same color
pub fn count_same(pixels: &[HSV]) -> usize {
    use std::collections::hash_map::HashMap;
    let mut m: HashMap<u8, usize> = HashMap::new();

    for i in pixels {
        let &HSV(_, s, v) = i;

        // only count colors, not black and white
        if v < 3e-3 || s < 3e-3 {
            continue;
        }

        let col = i.to_u8();
        let counter = m.entry(col).or_insert(0);
        *counter += 1;
    }

    *m.values().max().unwrap_or(&0)
}

/// `color_variance` is an ad-hoc measure for the interestingness of an image
pub fn color_variance(pixels: &[HSV]) -> f64 {
    // TODO: long term I want to try to replace this function by a neural network classifier
    // maybe using https://crates.io/crates/leaf ?
    let n = pixels.len() as f64;

    let num_black = pixels.iter()
                          .filter(|&&HSV(_, _, v)| v < 1e-3)
                          .count();
    let num_white = pixels.iter()
                          .filter(|&&HSV(_, s, v)| s < 1e-3 && v > 1e-3)
                          .count();

    // if more than 85% of pixels are black or white, reject
    info!("{:3.2}% of pixels are black or white", (num_black + num_white) as f64 / n * 100.);
    if (num_black + num_white) as f64 > 0.85 * n {
        return -1.
    }

    // if more than 50% of pixels have the same color, reject
    let num_same = count_same(pixels);
    info!("{:3.2}% of pixels have the same color", (num_same) as f64 / n * 100.);
    if num_same as f64 > 0.5 * n {
        return -2.
    }

    // otherwise use the variance as a method to estimate the interestingness
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

    let tmp = (var_h, if var_s > var_v {var_s} else {var_v});
    info!("hue: {}, sat: {}, value: {}", mean_h, mean_s, mean_v);
    info!("hue: {}, sat: {}, value: {}", var_h, var_s, var_v);
    (tmp.0 + tmp.1) / 2.
}
