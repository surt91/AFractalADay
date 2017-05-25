extern crate std;
extern crate num;
extern crate png;
extern crate rayon;
extern crate rand;
extern crate test;

use self::rand::Rng;
use self::num::complex::Complex;

use std::path::Path;
use std::fs::File;
use std::io;
// To use encoder.set()
use self::png::HasParameters;
use itertools::Itertools;

use self::rayon::prelude::*;

use functions::{Terms, Coef};

pub struct NewtonFractal {
    pub a: Coef,
    pub f: Box<Fn(Complex<f64>) -> Complex<f64> + Sync>,
    h: f64,
    rng: rand::StdRng,
    pub formula: String
}

pub struct Formula {
    pub f: Box<Fn(Complex<f64>) -> Complex<f64> + Sync>,
    pub a: Coef,
    pub formula: String,
    pub prefix: String
}


struct Convergence {
    count: i64,
    value: Complex<f64>
}

fn hsv2rgb(h: f64, s: f64, v: f64) -> (f64, f64, f64) {
    // https://de.wikipedia.org/wiki/HSV-Farbraum#Umrechnung_HSV_in_RGB

    let hi = (h * 6.).floor() as u32;
    let f = h * 6. - hi as f64;
    let p = v*(1.-s);
    let q = v*(1.-s*f);
    let t = v*(1.-s*(1.-f));

    match hi {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        6 => (v, t, p),
        _ => (0., 0., 0.)
    }
}

impl NewtonFractal {
    pub fn new(f: Option<Box<Fn(Complex<f64>) -> Complex<f64> + Sync>>, seed: Option<&[usize]>) -> NewtonFractal {
        let mut rng: rand::StdRng = match seed {
            Some(x) => rand::SeedableRng::from_seed(x),
            None => rand::StdRng::new().unwrap()
        };
        let formula = match f {
            Some(x) => Formula {f: x, a: Coef::Real(1.), formula: "n/a".to_string(), prefix: "Newton Fractal of".to_string()},
            None => NewtonFractal::random_formula(&mut rng)
        };

        NewtonFractal {a: formula.a, f: formula.f, h: 1e-4, rng, formula: formula.prefix + &formula.formula}
    }

    fn iterate(&self, mut state: Complex<f64>) -> Convergence {
        let mut ctr = 0;
        let threshold = 1e-4;
        let mut tmp;

        let kernel: Box<Fn(Complex<f64>) -> Complex<f64>> = match self.a {
            Coef::Complex(z) => Box::new(move |state| state - z * (self.f)(state) / self.fprime(state)),
            Coef::Real(x) => Box::new(move |state| state - x * (self.f)(state) / self.fprime(state))
        };
        // *attention*: this is a do while loop, mind that the "body" is actually the
        // condition and the body is empty, thus omitted
        while {
            tmp = state;
            state = kernel(state);
            ctr += 1;

            (state - tmp).norm() > threshold && ctr < 1000
        } {}
        Convergence {count: ctr, value: state}
    }

    fn fprime(&self, x: Complex<f64>) -> Complex<f64> {
        ((self.f)(x + self.h) - (self.f)(x - self.h)) / (2. * self.h)
    }

    pub fn random_formula(rng: &mut rand::StdRng) -> Formula{
        // use up to 5 terms but at least 2
        let num_terms = (rng.gen_range(0f64, 1.) * 3.).floor() as i32 + 2;
        let mut terms: Vec<Box<Fn(Complex<f64>) -> Complex<f64> + Sync>> = Vec::new();
        let mut term_string: Vec<String> = Vec::new();

        let mut prefix;
        let a_re = (rng.gen_range(1f64, 2.) * 10.).floor() / 10.;
        let a_im = (rng.gen_range(1f64, 2.) * 10.).floor() / 10.;
        let alpha = if rng.gen::<f64>() < 0.1 {
            let tmp = Complex::new(a_re, a_im);
            prefix = format!("Generalized Newton Fractal (a = {}) of ", tmp);
            Coef::Complex(tmp)
        } else if rng.gen::<f64>() < 0.4 {
            let tmp = a_re;
            prefix = format!("Generalized Newton Fractal (a = {}) of ", tmp);
            Coef::Real(tmp)
        } else {
            prefix = "Newton Fractal of ".to_string();
            Coef::Real(1.)
        };
        prefix += "z ↦ ";

        let a_real_gen = |generator: &mut rand::StdRng| 0.1f64.max((generator.gen_range(-1f64, 1f64) * 3. * 10.).round() / 10.);
        let a_comp_gen = |generator: &mut rand::StdRng| Complex::new(a_real_gen(generator), a_real_gen(generator));

        let mut possible_terms = Terms::new();

        for _ in 0..num_terms {
            // let a be a complex number in 30% of all cases
            let a = match rng.gen_range(0f64, 1.) {
                x if x < 0.3 => Coef::Complex(a_comp_gen(rng)),
                _ => Coef::Real(a_real_gen(rng))
            };

            let neo = possible_terms.choice(a, rng);
            terms.push(neo.0);
            term_string.push(neo.1);
        }

        let f = move |x| terms.iter()
                              .map(move |f| f(x))
                              .fold(Complex {re: 0., im: 0.}, |sum: Complex<f64>, x| sum + x);

        Formula {f: Box::new(f), a: alpha, formula: term_string.join(" + "), prefix: prefix}
    }

    fn raster(&self, x: i32, y: i32, xscale: f64, yscale: f64) -> Vec<Convergence> {
        let pixels: Vec<(i32, i32)> = iproduct!(0..y, 0..x).collect();
        pixels.par_iter()
              .map(|&(j, i)| {
                  let xp = (i-x/2) as f64 * xscale;
                  let yp = (j-y/2) as f64 * yscale;
                  let p = Complex {re: xp, im: yp};
                  let state = self.iterate(p);
                  state
              })
              .collect()
    }

    pub fn render(&mut self, resolution: (i32, i32), filename: &str) -> io::Result<i64> {
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

        let styles = [style_spooky, style_strong, style_vibrant, style_pastell];
        let style_names = ["spooky", "strong", "vibrant", "pastell"];
        let num_styles = styles.len();

        let idx = self.rng.gen_range(0, num_styles as usize);
        let style = styles[idx];
        info!("use style '{}'", style_names[idx]);
        info!("rcol {}", random_color);
        info!("rcnt {}", random_count);
        info!("rzo {}", random_zoom);

        let tmp_buffer: Vec<Vec<u8>> = states.par_iter()
                            .map(|i| {
                                let (h, s, v) = style(i.value, i.count,
                                                      Some(random_color), Some(random_count));

                                let (r, g, b) = hsv2rgb(h, s, v);
                                let a = 255;

                                vec![(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8, a]
                            })
                            .collect();
        let buffer: Vec<u8> = tmp_buffer.into_iter()
                                        .flatten()
                                        .collect();

        let path = Path::new(filename);
        let file = File::create(path)?;
        let ref mut w = io::BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, x as u32, y as u32);
        encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        writer.write_image_data(&buffer)?; // Save

        Ok(total_iterations)
    }
}

fn style_pastell(value: Complex<f64>, count: i64, random_color: Option<f64>, random_count: Option<f64>) -> (f64, f64, f64) {
    let random_color = random_color.unwrap_or(1.);
    let random_count = random_count.unwrap_or(1.);

    let hue = (value.norm() * 10. * random_color) % 1.;
    let value = 1f64;
    let tmp = count as f64 / (10. + 40. * random_count);
    let saturation = 1f64.min(tmp);

    (hue, saturation, value)
}

fn style_vibrant(value: Complex<f64>, count: i64, random_color: Option<f64>, random_count: Option<f64>) -> (f64, f64, f64) {
    let random_color = random_color.unwrap_or(1.);
    let random_count = random_count.unwrap_or(1.);

    let hue = (value.norm() * 10. * (random_color + 0.1)) % 1.;
    let value = 1f64;
    let tmp = count as f64 / (10. + 40. * random_count);
    let saturation = 1. - 1f64.min(tmp);

    (hue, saturation, value)
}

fn style_strong(value: Complex<f64>, count: i64, random_color: Option<f64>, random_count: Option<f64>) -> (f64, f64, f64) {
    let random_color = random_color.unwrap_or(1.);
    let random_count = random_count.unwrap_or(1.);

    let hue = (value.norm() * 10. * random_color) % 1.;
    let saturation = 1f64;
    let tmp = count as f64 / (10. + 100. * random_count);
    let value = 1f64.min(tmp.powf(0.7));

    (hue, saturation, value)
}

fn style_spooky(value: Complex<f64>, count: i64, random_color: Option<f64>, random_count: Option<f64>) -> (f64, f64, f64) {
    let random_color = random_color.unwrap_or(1.);
    let random_count = random_count.unwrap_or(1.);

    let hue = (value.norm() * 10. * random_color) % 1.;
    let saturation = 1f64;
    let tmp = count as f64 / (10. + 50. * random_count);
    let value = 1f64.min(tmp);

    (hue, saturation, value)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsv2rgb_red() {
        assert_eq!((1., 0., 0.), hsv2rgb(0., 1., 1.));
    }
    #[test]
    fn test_hsv2rgb_yellow() {
        assert_eq!((1., 1., 0.), hsv2rgb(60./360., 1., 1.));
    }
    #[test]
    fn test_hsv2rgb_brown() {
        assert_eq!((0.36, 0.18, 0.09), hsv2rgb(20./360., 0.75, 0.36));
    }
    #[test]
    fn test_hsv2rgb_darkgreen() {
        assert_eq!((0., 0.5, 0.), hsv2rgb(120./360., 1., 0.5));
    }
    #[test]
    fn test_hsv2rgb_orange() {
        assert_eq!((1., 0.5, 0.), hsv2rgb(30./360., 1., 1.));
    }
    #[test]
    fn test_hsv2rgb_safran() {
        assert_eq!((1., 0.75, 0.), hsv2rgb(45./360., 1., 1.));
    }
    #[test]
    fn test_hsv2rgb_indigo() {
        assert_eq!((0.25, 0., 1.), hsv2rgb(255./360., 1., 1.));
    }
}
