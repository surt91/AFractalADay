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

use functions::{Terms, Coef, Real, Cplx, ComplexFunction, Formula};
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
            None => NewtonFractal::random_formula(&mut rng)
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
            Coef::Real(x) if (1. - x).abs() < 1e-4 => "Newton Fractal".to_string(),
            Coef::Real(x) => format!("Generalized Newton Fractal (x = {}) of ", x),
            Coef::Complex(y) => format!("Generalized Newton Fractal (x = {}) of ", y)
        };
        description += &f.readable;

        NewtonFractal {
            a,
            f: f.callable,
            description,
            h: 1e-4,
            rng,
            style
        }
    }
}

pub struct NewtonFractal {
    pub a: Coef,
    pub f: ComplexFunction,
    h: Real,
    rng: rand::StdRng,
    pub description: String,
    style: Style
}

impl NewtonFractal {
    pub fn style(&mut self, s: Style) {
        self.style = s;
    }

    fn iterate(&self, mut state: Cplx) -> Convergence {
        let mut ctr = 0;
        let threshold = 1e-4;
        let mut tmp;

        let kernel: Box<Fn(Cplx) -> Cplx> = match self.a {
            Coef::Complex(z) => Box::new(move |state| state - z * (self.f)(state) / self.fprime(state)),
            Coef::Real(x) => Box::new(move |state| state - x * (self.f)(state) / self.fprime(state))
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

    fn fprime(&self, x: Cplx) -> Cplx {
        ((self.f)(x + self.h) - (self.f)(x - self.h)) / (2. * self.h)
    }

    fn random_coef(rng: &mut rand::StdRng) -> Coef {
        let a_re = (rng.gen_range(1. as Real, 2.) * 10.).floor() / 10.;
        let a_im = (rng.gen_range(1. as Real, 2.) * 10.).floor() / 10.;
        if rng.gen::<f64>() < 0.1 {
            let tmp = Complex::new(a_re, a_im);
            // prefix = format!("Generalized Newton Fractal (a = {}) of ", tmp);
            Coef::Complex(tmp)
        } else if rng.gen::<f64>() < 0.4 {
            let tmp = a_re;
            // prefix = format!("Generalized Newton Fractal (a = {}) of ", tmp);
            Coef::Real(tmp)
        } else {
            // prefix = "Newton Fractal of ".to_string();
            Coef::Real(1.)
        }
    }

    fn random_formula(rng: &mut rand::StdRng) -> Formula {
        // use up to 4 terms but at least 1
        let num_terms = (rng.gen_range(0f64, 1.) * 3.).floor() as i32 + 1;
        let mut terms: Vec<ComplexFunction> = Vec::new();
        let mut term_string: Vec<String> = Vec::new();

        let a_real_gen = |generator: &mut rand::StdRng| (generator.gen_range(-1. as Real, 1.) * 3. * 10.).round() / 10.;
        let a_comp_gen = |generator: &mut rand::StdRng| Complex::new(a_real_gen(generator), a_real_gen(generator));

        let mut possible_terms = Terms::new();
        // chance that all coefficients will be real
        let always_real = rng.gen_range(0f64, 1.) < 0.5;

        for _ in 0..num_terms {
            // let a be a complex number in 30% of all cases
            let a = if !always_real && rng.gen_range(0f64, 1.) < 0.3 {
                        Coef::Complex(a_comp_gen(rng))
                    } else {
                        Coef::Real(a_real_gen(rng))
                    };

            let neo = possible_terms.choice(a, rng);
            terms.push(neo.callable);
            term_string.push(neo.readable);
        }

        let f = move |x| terms.iter()
                              .map(move |f| f(x))
                              .fold(Complex {re: 0., im: 0.}, |sum, x| sum + x);

        Formula {callable: Box::new(f),
                 readable: "z ↦ ".to_string() + &term_string.join(" + ")}
    }

    pub fn raster(&self, x: i32, y: i32, xscale: f64, yscale: f64) -> Vec<Convergence> {
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

    // color_variance is an ad-hoc measure for the interestingness of an image
    fn color_variance(pixels: &[color::HSV]) -> f64 {
        let n = pixels.len() as f64;
        let mean_h = 1./n * pixels.iter()
                                  .map(|&color::HSV(h, _, _)| h)
                                  .sum::<f64>();
        let mean_s = 1./n * pixels.iter()
                                  .map(|&color::HSV(_, s, _)| s)
                                  .sum::<f64>();
        let mean_v = 1./n * pixels.iter()
                                  .map(|&color::HSV(_, _, v)| v)
                                  .sum::<f64>();

        let var_h = 1./n * pixels.iter()
                                 .map(|&color::HSV(h, _, _)| (h-mean_h) * (h-mean_h))
                                 .sum::<f64>();
        let var_s = 1./n * pixels.iter()
                                 .map(|&color::HSV(_, s, _)| (s-mean_s) * (s-mean_s))
                                 .sum::<f64>();
        let var_v = 1./n * pixels.iter()
                                 .map(|&color::HSV(_, _, v)| (v-mean_v) * (v-mean_v))
                                 .sum::<f64>();


        let tmp = (var_h, if var_s > var_v {var_s} else {var_v});
        if tmp.0 < tmp.1 {tmp.0} else {tmp.1}
    }

    pub fn render(&mut self, resolution: (i32, i32), filename: &str) -> io::Result<f64> {
        let (x, y) = resolution;

        // use randomness to determine the colors
        let random_color = self.rng.gen_range(0f64, 1.);
        let random_count = self.rng.gen_range(0f64, 1.);
        let random_zoom = self.rng.gen_range(0.1f64, 2.);
        let scale = 4e-3 * random_zoom;

        let states = self.raster(x, y, scale, scale);
        let total_iterations: i64 = states.par_iter()
                                     .map(|i| i.count)
                                     .sum();
        info!("{:.2}M iterations", total_iterations as f64/1e6);

        let style = Style::index(self.rng.gen_range(0, Style::num()));
        info!("use style '{}'", style);
        info!("rcol {}", random_color);
        info!("rcnt {}", random_count);
        info!("rzo {}", random_zoom);

        let hsv: Vec<color::HSV> = states.par_iter()
                                         .map(|i| (style.callable)(i, Some(random_color), Some(random_count)))
                                         .collect();

        let var = NewtonFractal::color_variance(&hsv);
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
