extern crate std;
extern crate num;
extern crate png;
extern crate rayon;
extern crate rand;

use self::rand::Rng;
use self::num::complex::Complex;

use std::path::Path;
use std::fs::File;
use std::io;
// To use encoder.set()
use self::png::HasParameters;
use itertools::Itertools;

use self::rayon::prelude::*;

use functions::{Coef, Real, Cplx, ComplexFunction, Formula, derivative, random_formula};
use color;

use style::Style;

pub struct Convergence {
    pub count: i64,
    pub value: Cplx
}

pub struct NewtonFractalBuilder {
    a: Option<Coef>,
    f: Option<Formula>,
    seed: Option<usize>,
    style: Option<Style>
}

// Builder Pattern to create a NewtonFractal
impl NewtonFractalBuilder {
    pub fn new() -> NewtonFractalBuilder {
        NewtonFractalBuilder {
            a: None,
            f: None,
            seed: None,
            style: None
        }
    }

    pub fn coefficient(mut self, a: Coef) -> NewtonFractalBuilder {
        self.a = Some(a);
        self
    }

    pub fn formula(mut self, f: Formula) -> NewtonFractalBuilder {
        self.f = Some(f);
        self
    }

    pub fn seed(mut self, seed: usize) -> NewtonFractalBuilder {
        self.seed = Some(seed);
        self
    }

    pub fn style(mut self, style: Style) -> NewtonFractalBuilder {
        self.style = Some(style);
        self
    }

    pub fn build(self) -> NewtonFractal {
        let mut rng: rand::StdRng = match self.seed {
            Some(x) => { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) },
            None => rand::StdRng::new().unwrap()
        };

        // fill in defaults, if members are not given
        // most defaults will be random
        let f = match self.f {
            Some(x) => x,
            None => random_formula(&mut rng)
        };

        let a = match self.a {
            Some(x) => x,
            None => NewtonFractal::random_coef(&mut rng)
        };

        let style = match self.style {
            Some(x) => x,
            None => Style::random_style(&mut rng)
        };


        let mut description = match a {
            Coef::Real(x) if (1. - x).abs() < 1e-4 => "Newton Fractal of ".to_string(),
            Coef::Real(x) => format!("Generalized Newton Fractal (x = {}) of ", x),
            Coef::Complex(y) => format!("Generalized Newton Fractal (x = {}) of ", y)
        };
        description += &f.readable;

        info!("Will render {}", description);

        NewtonFractal {
            a,
            f: f.callable,
            description,
            rng,
            style
        }
    }
}

pub struct NewtonFractal {
    a: Coef,
    f: ComplexFunction,
    rng: rand::StdRng,
    pub description: String,
    style: Style
}

impl IteratedFractal for NewtonFractal {
    fn iterate(&self, mut state: Cplx) -> Convergence {
        let mut ctr = 0;
        let threshold = 1e-4;
        let mut tmp;

        let kernel: Box<Fn(Cplx) -> Cplx> = match self.a {
            Coef::Complex(z) => Box::new(move |state| state - z * (self.f)(state) / derivative(&self.f, &state)),
            Coef::Real(x) => Box::new(move |state| state - x * (self.f)(state) / derivative(&self.f, &state))
        };
        // this is a do while loop, mind that the "body" is actually the
        // condition and the body is empty, thus omitted
        while {
            tmp = state;
            state = kernel(state);
            ctr += 1;

            (state - tmp).norm() > threshold && ctr < 1000 && !state.re.is_nan() && !state.im.is_nan()
        } {}
        Convergence {count: ctr, value: state}
    }

    fn get_rng(&mut self) -> &mut rand::StdRng {
        &mut self.rng
    }

    fn get_style(&self) -> &Style {
        &self.style
    }
}

/// The IteratedFractal trait applies to all ``Julia set type'' fractals, i.e., all fractals
/// that can be visualized by assigning every pixel a color dependent on a value and an iteration
/// count.
pub trait IteratedFractal : Sync {
    fn iterate(&self, state: Cplx) -> Convergence;
    fn get_rng(&mut self) -> &mut rand::StdRng;
    fn get_style(&self) -> &Style;

    fn random_coef(rng: &mut rand::StdRng) -> Coef {
        let a_re = (rng.gen_range(1. as Real, 2.) * 10.).floor() / 10.;
        let a_im = (rng.gen_range(1. as Real, 2.) * 10.).floor() / 10.;
        if rng.gen::<f64>() < 0.1 {
            let tmp = Complex::new(a_re, a_im);
            Coef::Complex(tmp)
        } else if rng.gen::<f64>() < 0.4 {
            let tmp = a_re;
            Coef::Real(tmp)
        } else {
            Coef::Real(1.)
        }
    }

    fn raster(&self, x: i32, y: i32, xscale: f64, yscale: f64) -> Vec<Convergence> {
        let pixels: Vec<(i32, i32)> = iproduct!(0..y, 0..x).collect();
        pixels.par_iter()
              .map(|&(j, i)| {
                  let xp = (i-x/2) as f64 * xscale;
                  let yp = (j-y/2) as f64 * yscale;
                  let p = Complex {re: xp as Real, im: yp as Real};
                  self.iterate(p)
              })
              .collect()
    }

    fn render(&mut self, resolution: (i32, i32), filename: &str) -> io::Result<f64> {
        let (x, y) = resolution;

        // use randomness to determine the colors
        let random_color = self.get_rng().gen_range(0f64, 1.);
        let random_count = self.get_rng().gen_range(0f64, 1.);
        let random_zoom = self.get_rng().gen_range(0.1f64, 2.);
        let scale = 4e-3 * random_zoom;

        let states = self.raster(x, y, scale, scale);
        let total_iterations: i64 = states.par_iter()
                                     .map(|i| i.count)
                                     .sum();
        info!("{:.2}M iterations", total_iterations as f64/1e6);

        info!("use style '{}'", self.get_style());
        info!("rcol {}", random_color);
        info!("rcnt {}", random_count);
        info!("rzo {}", random_zoom);

        let hsv: Vec<color::HSV> = states.par_iter()
                                         .map(|i| (self.get_style().callable)(i, Some(random_color), Some(random_count)))
                                         .collect();

        let var = color::color_variance(&hsv);
        info!("variance: {}", var);

        let tmp_buffer: Vec<Vec<u8>> = hsv.par_iter()
                            .map(|hsv| {
                                let color::RGB(r, g, b) = hsv.to_rgb();
                                let a = 255;

                                vec![(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8, a]
                            })
                            .collect();
        let buffer: Vec<u8> = tmp_buffer.into_iter()
                                        .flatten()
                                        .collect();

        let tmp = filename;
        let path = Path::new(&tmp);
        let file = File::create(path)?;
        let w = io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, x as u32, y as u32);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        writer.write_image_data(&buffer)?; // Save

        Ok(var)
    }
}
