use std::f32;
use numbers::Real;
use color::{RGB, RGBA};

extern crate rayon;
use self::rayon::prelude::*;

/// data structure containing a 2d-histogram with 4 channels (rgba)
pub struct ColoredHistogram {
    resolution: (u32, u32),
    bounds: (f32, f32, f32, f32),
    bins: Vec<(f64, f64, f64, u64)>
}

impl ColoredHistogram {
    /// create a new empty histogram
    ///
    /// # Arguments
    ///
    /// * `resolution` - number of bins in x and y direction
    /// * `bounds` - minimum and maximum values, i.e., range of the histogram
    pub fn new(resolution: (u32, u32), bounds: (f32, f32, f32, f32)) -> ColoredHistogram {
        let (x_res, y_res) = resolution;

        let bins = vec![(0., 0., 0., 0u64); (x_res*y_res) as usize];

        ColoredHistogram {
            resolution,
            bounds,
            bins
        }
    }

    /// normalize the four channels of the histogram to RGBA values, with a gamma correction
    pub fn normalize(&self) -> Vec<RGBA> {
        let max_a = self.bins.par_iter()
                            .map(|&(_, _, _, a)| a)
                            .max()
                            .unwrap() as f64;
        let max_a = max_a.ln();

        // normalize
        let gamma = 4.;
        self.bins.par_iter()
            .map(|&(r, g, b, a)| {
                let norm = 1. / a as f64;
                let r = ((r*norm).powf(1./gamma) * 255.) as u8;
                let g = ((g*norm).powf(1./gamma) * 255.) as u8;
                let b = ((b*norm).powf(1./gamma) * 255.) as u8;
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
        let scale = if x_w > y_w {x_w} else {y_w};
        let x_offset = if x_w > y_w*aspect {0.} else {(y_w*aspect - x_w)/2. };
        let y_offset = if y_w*aspect > x_w {0.} else {(x_w - y_w)/2.};

        for i in values {
            let (z, c) = i;
            let x = ((z[0] - min_x + x_offset) / scale * (x_res-1) as f32 / aspect) as usize;
            let y = ((z[1] - min_y + y_offset) / scale * (y_res-1) as f32) as usize;
            // discard points outside
            if y*x_res as usize + x < self.bins.len() {
                let RGB(r, g, b) = c;
                self.bins[y*x_res as usize + x].0 += r;
                self.bins[y*x_res as usize + x].1 += g;
                self.bins[y*x_res as usize + x].2 += b;
                self.bins[y*x_res as usize + x].3 += 1;
            }
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

/// finds bounds containing 90% of all input points
///
/// # Arguments
///
/// * `vals` - Iterator yielding coordinates
pub fn bounds90<'a, I>(vals: I) -> (f32, f32, f32, f32)
    where I: Iterator<Item=&'a [Real; 2]>
{
    let mut rs: Vec<&[Real;2]> = vals.collect();
    let n = rs.len();

    rs.sort_by(|r1, r2| r1[0].partial_cmp(&r2[0]).unwrap());
    let min_x = rs[n / 10][0];
    let max_x = rs[n - n / 10 - 1][0];

    rs.sort_by(|r1, r2| r1[1].partial_cmp(&r2[1]).unwrap());
    let min_y = rs[n / 10][1];
    let max_y = rs[n - n / 10 - 1][1];

    // FIXME
    // it is possible that we get nan values
    // until I have a better idea, just choose the something arbitrary
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
    let scale = if x_w > y_w {x_w} else {y_w};
    let x_offset = if x_w > y_w*aspect {0.} else {(y_w*aspect - x_w)/2. };
    let y_offset = if y_w*aspect > x_w {0.} else {(x_w - y_w)/2.};

    let mut out = vec![0; (x_res*y_res) as usize];
    for z in vals {
        let x = ((z[0] - min_x + x_offset) / scale * (x_res-1) as f32 / aspect) as usize;
        let y = ((z[1] - min_y + y_offset) / scale * (y_res-1) as f32) as usize;
        // discard points outside
        if y*x_res as usize + x < out.len() {
            out[y*x_res as usize + x] += 1;
        }
    }

    out
}
