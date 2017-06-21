mod heighway_dragon;
pub mod iterated_function_system_builder;

extern crate rand;
use self::rand::Rng;

extern crate png;
use self::png::HasParameters;

use std::{f32, f64};
use itertools::Itertools;

use std::io;
use std::path::Path;
use std::fs::File;

use numbers::{Real, Cplx};
use color;

/// The `IteratedFunctionSystem` trait applies to all ``Chaos Game type'' fractals.
pub trait IteratedFunctionSystem : Sync {
    fn description(&self) -> &str;
    fn iterate(&mut self) -> Vec<Cplx>;
    fn get_rng(&mut self) -> &mut rand::StdRng;

    fn raster(&self, resolution: (u32, u32), values: Vec<Cplx>) -> Vec<u64> {
        let X = resolution.0;
        let Y = resolution.1;
        let (min_x, max_x, min_y, max_y) = values.iter()
                .fold((f32::INFINITY, -f32::INFINITY, f32::INFINITY, -f32::INFINITY),
                    |mut extrema, z| {
                        if extrema.0 > z.re as f32 {
                            extrema.0 = z.re as f32
                        }
                        if extrema.1 < z.re as f32 {
                            extrema.1 = z.re as f32
                        }
                        if extrema.2 > z.im as f32 {
                            extrema.2 = z.im as f32
                        }
                        if extrema.3 < z.im as f32 {
                            extrema.3 = z.im as f32
                        }
                        extrema
                    }
                );

        let mut out = vec![0; (X*Y) as usize];

        for z in values {
            let x = ((z.re - min_x) / (max_x - min_x) * (X-1) as f32) as usize;
            let y = ((z.im - min_y) / (max_y - min_y) * (Y-1) as f32) as usize;
            out[y*X as usize + x] += 1;
        }

        out
    }

    // TODO: implement supersampling
    fn render(&mut self, resolution: (u32, u32),
                         scale: Option<f64>,
                         center: Option<(f64, f64)>,
                         filename: &str) -> io::Result<f64> {
        let scale = match scale {
            Some(x) => (x, x),
            None => {
                let x = 1. / resolution.1 as f64;
                (x, x)
            }
        };

        let center = match center {
            Some(x) => x,
            None => (0., 0.)
        };

        let values = self.iterate();
        let states = self.raster(resolution, values);

        let max_val = states.iter().max().unwrap();
        // TODO: maybe color by distance of the jump?
        let hsv: Vec<color::HSV> = states.iter()
                                         .map(|i| color::HSV(0., 0., *i as f64 / *max_val as f64))
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

        // TODO: save in an extra .png method
        let tmp = filename;
        let path = Path::new(&tmp);
        let file = File::create(path)?;
        let w = io::BufWriter::new(file);

        let (x, y) = resolution;
        let mut encoder = png::Encoder::new(w, x as u32, y as u32);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        writer.write_image_data(&buffer)?; // Save

        Ok(var)
    }
}
