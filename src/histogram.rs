use std::f32;
use numbers::Real;
use color::{RGB, RGBA};
use itertools::multizip;

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
    bounds.0 *= 1.05;
    bounds.1 *= 1.05;
    bounds.2 *= 1.05;
    bounds.3 *= 1.05;
    bounds
}

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

pub fn histogram_colored<I>(vals: I, resolution: (u32, u32), bounds: (f32, f32, f32, f32)) -> Vec<RGBA>
    where I: Iterator<Item=([Real; 2], RGB)>
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

    let mut r_out = vec![0.; (x_res*y_res) as usize];
    let mut g_out = vec![0.; (x_res*y_res) as usize];
    let mut b_out = vec![0.; (x_res*y_res) as usize];
    let mut a_out = vec![0u64; (x_res*y_res) as usize];

    for i in vals {
        let (z, c) = i;
        let x = ((z[0] - min_x + x_offset) / scale * (x_res-1) as f32 / aspect) as usize;
        let y = ((z[1] - min_y + y_offset) / scale * (y_res-1) as f32) as usize;
        // discard points outside
        if y*x_res as usize + x < a_out.len() {
            let RGB(r, g, b) = c;
            r_out[y*x_res as usize + x] += r;
            g_out[y*x_res as usize + x] += g;
            b_out[y*x_res as usize + x] += b;
            a_out[y*x_res as usize + x] += 1;
        }
    }

    let max_a = a_out.iter().max().unwrap();
    let max_a = (*max_a as f64).ln();

    // normalize
    let gamma = 4.;
    multizip((&r_out, &g_out, &b_out, &a_out))
        .map(|(r, g, b, a)| {
            let n = 1. / *a as f64;
            let r = ((r*n).powf(1./gamma) * 255.) as u8;
            let g = ((g*n).powf(1./gamma) * 255.) as u8;
            let b = ((b*n).powf(1./gamma) * 255.) as u8;
            let a = ((*a as f64).ln() / max_a * 255.) as u8;
            RGBA(r, g, b, a)
        }
    ).collect()
}
