mod heighway_dragon;
mod barnsly_fern;
pub mod iterated_function_system_builder;

extern crate rand;

use std::{f32, f64};
use std::io;
use itertools::Itertools;

use numbers::Real;
use color;
use png;

fn bounds<'a, I>(vals: I) -> (f32, f32, f32, f32)
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

fn histogram<I>(vals: I, resolution: (u32, u32), bounds: (f32, f32, f32, f32)) -> Vec<usize>
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

/// The `IteratedFunctionSystem` trait applies to all ``Chaos Game type'' fractals.
pub trait IteratedFunctionSystem : Sync + Iterator<Item=[Real; 2]> {
    fn description(&self) -> &str;
    fn get_rng(&mut self) -> &mut rand::StdRng;

    // TODO: implement supersampling
    fn render(&mut self, resolution: (u32, u32),
                         samples_per_pixel: usize,
                         filename: &str) -> io::Result<f64> {
        let (x, y) = resolution;

        // warm up and get sample to derive bounds
        let values: Vec<[Real; 2]> = self.skip(100).take((x * y) as usize).collect();
        // read bounds from sample
        let b = bounds(values.iter());
        // generate histogram, using the sample and new values
        let hist = histogram(values.into_iter()
                                   .chain(
                                       self.take((samples_per_pixel-1) * (x * y) as usize)
                                   ),
                             resolution, b);

        let max_val = hist.iter().max().unwrap();
        // TODO: maybe color by distance of the jump?
        // TODO: implement fractal flame (http://flam3.com/flame_draves.pdf)
        // TODO: use color (multiple histograms for color channels + alpha (total count))
        let hsv: Vec<color::HSV> = hist.iter()
                                         .map(|i| color::HSV(0., 0., (*i as f64).ln() / (*max_val as f64).ln()))
                                         .collect();

        let var = color::color_variance(&hsv);
        info!("variance: {}", var);

        let buffer: Vec<u8> = hsv.iter()
                                 .map(|hsv| {
                                     let color::RGB(r, g, b) = hsv.to_rgb();
                                     let a = 255;

                                     vec![(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8, a]
                                 })
                                 .flatten()
                                 .collect();

        png::save_png(filename, x, y, &buffer)?;
        Ok(var)
    }
}
