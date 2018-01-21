use std::f32;
use numbers::Real;
use color::{RGB, RGBA};

extern crate rayon;
use self::rayon::prelude::*;

use std::cmp::Ordering;

/// data structure containing a 2d-histogram with 4 channels (rgba)
#[derive(Clone)]
pub struct ColoredHistogram {
    resolution: (u32, u32),
    bounds: (f32, f32, f32, f32),
    bins: Vec<(f64, f64, f64, u64)>,
    gamma: f64,
    vibrancy: f64,
}

impl ColoredHistogram {
    /// create a new empty histogram
    ///
    /// # Arguments
    ///
    /// * `resolution` - number of bins in x and y direction
    /// * `bounds` - minimum and maximum values, i.e., range of the histogram
    pub fn new(
        resolution: (u32, u32),
        bounds: (f32, f32, f32, f32),
        vibrancy: f64,
        gamma: f64
    )
    -> ColoredHistogram
    {
        let (x_res, y_res) = resolution;

        let bins = vec![(0., 0., 0., 0u64); (x_res*y_res) as usize];

        ColoredHistogram {
            resolution,
            bounds,
            bins,
            gamma,
            vibrancy
        }
    }

    /// apply gamma correction and vibrancy
    fn apply_vibrancy_and_gamma(&self, color: f64, alpha: u64) -> u8 {
        let norm = 1. / alpha as f64;

        let first = (self.vibrancy * color * norm * 255.) as u8;
        let second = ((1.-self.vibrancy) * (color*norm).powf(1./self.gamma) * 255.) as u8;

        first + second
    }

    /// normalize the four channels of the histogram to RGBA values, with a gamma correction
    pub fn normalize(&self) -> Vec<RGBA> {
        let max_a = self.bins.par_iter()
                            .map(|&(_, _, _, a)| a)
                            .max()
                            .unwrap() as f64;
        let max_a = max_a.ln();

        // normalize
        self.bins.par_iter()
            .map(|&(r, g, b, a)| {
                let r = self.apply_vibrancy_and_gamma(r, a);
                let g = self.apply_vibrancy_and_gamma(g, a);
                let b = self.apply_vibrancy_and_gamma(b, a);
                let a = ((a as f64).ln() / max_a * 255.) as u8;
                RGBA(r, g, b, a)
            }
        ).collect()
    }

    /// merge another histogram into this histogram
    ///
    /// # Arguments
    ///
    /// * `other` - the histogram to merge
    pub fn merge(&mut self, other: &ColoredHistogram) {
        assert_eq!(self.resolution, other.resolution);
        assert_eq!(self.bounds, other.bounds);
        for i in 0..other.bins.len() {
            self.bins[i].0 += other.bins[i].0;
            self.bins[i].1 += other.bins[i].1;
            self.bins[i].2 += other.bins[i].2;
            self.bins[i].3 += other.bins[i].3;
        }
    }

    /// consume an iterator of coordinates and add them to the histogram
    ///
    /// # Arguments
    ///
    /// * `values` - iterator of coordinates
    pub fn feed<I>(&mut self, values: I)
        where I: Iterator<Item=([Real; 2], RGB)>
    {
        let (min_x, max_x, min_y, max_y) = self.bounds;
        let (x_res, y_res) = self.resolution;
        let aspect = x_res as f32 / y_res as f32;

        // keep aspect ratio and center the fractal
        let x_w = max_x - min_x;
        let y_w = max_y - min_y;
        let x_offset;
        let y_offset;
        let x_scale;
        let y_scale;
        if x_w > y_w*aspect {
            x_offset = 0.;
            y_offset = (x_w/aspect - y_w)/2.;
            x_scale = x_w;
            y_scale = x_w / aspect;
        } else {
            x_offset = (y_w*aspect - x_w)/2.;
            y_offset = 0.;
            x_scale = y_w * aspect;
            y_scale = y_w;
        };

        for i in values {
            let (z, c) = i;

            // discard data outside of bounds
            if z[0] < min_x || z[0] > max_x || z[1] < min_y || z[1] > max_y || z[0].is_nan() || z[1].is_nan() {
                continue
            }

            let x = ((z[0] - min_x + x_offset) / x_scale * (x_res-1) as f32) as usize;
            let y = ((z[1] - min_y + y_offset) / y_scale * (y_res-1) as f32) as usize;

            let idx = y*x_res as usize + x;
            let RGB(r, g, b) = c;
            self.bins[idx].0 += r;
            self.bins[idx].1 += g;
            self.bins[idx].2 += b;
            self.bins[idx].3 += 1;
        }
    }
}

/// find bounds containing all input points
///
/// # Arguments
///
/// * `vals` - Iterator yielding coordinates
pub fn bounds<'a, I>(vals: I) -> (f32, f32, f32, f32)
    where I: Iterator<Item=&'a [Real; 2]>
{
    let mut bounds = vals.fold((f32::INFINITY, -f32::INFINITY, f32::INFINITY, -f32::INFINITY),
        |mut extrema, z| {
            if extrema.0 > z[0] as f32 {
                extrema.0 = z[0] as f32
            }
            if extrema.1 < z[0] as f32 {
                extrema.1 = z[0] as f32
            }
            if extrema.2 > z[1] as f32 {
                extrema.2 = z[1] as f32
            }
            if extrema.3 < z[1] as f32 {
                extrema.3 = z[1] as f32
            }
            extrema
        }
    );
    // 5% more
    bounds.0 -= 0.05 * (bounds.1-bounds.0);
    bounds.1 += 0.05 * (bounds.1-bounds.0);
    bounds.2 -= 0.05 * (bounds.3-bounds.2);
    bounds.3 += 0.05 * (bounds.3-bounds.2);

    bounds
}

/// finds bounds disarding a number of outliers in each direction
///
/// # Arguments
///
/// * `vals` - Iterator yielding coordinates
/// * `outliers` - Number of outliers to discard in each direction
pub fn bounds_without_outliers<'a, I>(vals: I, outliers: usize) -> (f32, f32, f32, f32)
    where I: Iterator<Item=&'a [Real; 2]>
{
    let mut rs: Vec<&[Real;2]> = vals.collect();
    let n = rs.len();

    // FIXME ignore NaN... this might lead to unexpected results
    rs.sort_by(|r1, r2| r1[0].partial_cmp(&r2[0]).unwrap_or(Ordering::Equal));
    let min_x = rs[outliers][0];
    let max_x = rs[n - outliers - 1][0];

    rs.sort_by(|r1, r2| r1[1].partial_cmp(&r2[1]).unwrap_or(Ordering::Equal));
    let min_y = rs[outliers][1];
    let max_y = rs[n - outliers - 1][1];

    // FIXME
    // it is possible that we get nan values
    // until I have a better idea, just choose something arbitrary
    let mut bounds = if min_x.is_finite() && max_x.is_finite() && min_y.is_finite() && max_y.is_finite() {
        (min_x, max_x, min_y, max_y)
    } else {
        (-1., 1., -1., 1.)
    };

    // 5% more
    bounds.0 -= 0.05 * (bounds.1-bounds.0);
    bounds.1 += 0.05 * (bounds.1-bounds.0);
    bounds.2 -= 0.05 * (bounds.3-bounds.2);
    bounds.3 += 0.05 * (bounds.3-bounds.2);

    bounds
}

/// finds bounds centering on the median filling the given aspect ratio
///
/// # Arguments
///
/// * `vals` - Iterator yielding coordinates
/// * `aspect` - width/height
pub fn bounds_zoom<'a, I>(vals: I, aspect: f32) -> (f32, f32, f32, f32)
    where I: Iterator<Item=&'a [Real; 2]>
{
    let mut rs: Vec<&[Real;2]> = vals.collect();
    let n = rs.len();
    let n5 = (n as f32 * 0.05) as usize;

    // FIXME ignore NaN... this might lead to unexpected results
    rs.sort_by(|r1, r2| r1[0].partial_cmp(&r2[0]).unwrap_or(Ordering::Equal));
    let min_x = rs[n5][0];
    let mut med_x = rs[n/2][0];
    let max_x = rs[n - n5 - 1][0];

    rs.sort_by(|r1, r2| r1[1].partial_cmp(&r2[1]).unwrap_or(Ordering::Equal));
    let min_y = rs[n5][1];
    let mut med_y = rs[n/2][1];
    let max_y = rs[n - n5 - 1][1];

    // FIXME
    // it is possible that we get nan values
    // until I have a better idea, just choose something arbitrary
    let mut bounds = if min_x.is_finite() && max_x.is_finite() && min_y.is_finite() && max_y.is_finite() {
        (min_x, max_x, min_y, max_y)
    } else {
        med_x = 0.;
        med_y = 0.;
        (-1., 1., -1., 1.)
    };

    let (min_x, max_x, min_y, max_y) = bounds;

    if max_x - min_x < aspect * (max_y - min_y) {
        bounds.3 = med_y + (max_x - min_x) / aspect / 2.;
        bounds.2 = med_y - (max_x - min_x) / aspect / 2.;
    } else {
        bounds.1 = med_x + (max_y - min_y) * aspect / 2.;
        bounds.0 = med_x - (max_y - min_y) * aspect / 2.;
    }

    // 5% more
    bounds.0 -= 0.05 * (bounds.1-bounds.0);
    bounds.1 += 0.05 * (bounds.1-bounds.0);
    bounds.2 -= 0.05 * (bounds.3-bounds.2);
    bounds.3 += 0.05 * (bounds.3-bounds.2);

    bounds
}

/// generates a 2d-histogram from an iterator
///
/// # Arguments
///
/// * `vals` - Iterator yielding coordinates
/// * `resolution` - number of bins in x and y direction
/// * `bounds` - minimum and maximum values, i.e., range of the histogram
pub fn histogram<I>(vals: I, resolution: (u32, u32), bounds: (f32, f32, f32, f32)) -> Vec<usize>
    where I: Iterator<Item=[Real; 2]>
{
    let (min_x, max_x, min_y, max_y) = bounds;
    let x_res = resolution.0;
    let y_res = resolution.1;
    let aspect = x_res as f32 / y_res as f32;

    // keep aspect ratio and center the fractal
    let x_w = max_x - min_x;
    let y_w = max_y - min_y;
    let x_offset = if x_w >= y_w*aspect {0.} else {(y_w*aspect - x_w)/2. };
    let y_offset = if y_w*aspect >= x_w {0.} else {(x_w - y_w)/2.};

    let mut out = vec![0; (x_res*y_res) as usize];
    for z in vals {
        // discard data outside of bounds
        if z[0] < min_x || z[0] > max_x || z[1] < min_y || z[1] > max_y {
            continue
        }

        let x = ((z[0] - min_x + x_offset) / x_w * (x_res-1) as f32) as usize;
        let y = ((z[1] - min_y + y_offset) / (x_w / aspect) * (y_res-1) as f32) as usize;

        if y*x_res as usize + x < out.len() {
            out[y*x_res as usize + x] += 1;
        }
    }

    out
}

pub fn histogram1d<I>(vals: I, bounds: (usize, usize)) -> Vec<f64>
    where I: Iterator<Item=usize>
{
    let (min, max) = bounds;
    let mut counts = vec![0usize; max - min + 1];
    let mut ctr = 0;
    for z in vals {
        // discard data outside of bounds
        if z < min || z > max {
            continue
        }
        counts[z] += 1;
        ctr += 1;
    }

    counts.iter()
          .map(|&x| x as f64 / ctr as f64)
          .collect()
}
