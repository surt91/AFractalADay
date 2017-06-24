extern crate std;
extern crate num;

extern crate rand;
use self::rand::Rng;

use numbers::Real;
use colored_ifs::ColoredIFS;
use color::RGB;

use super::colored_ifs_builder::ColoredIFSBuilder;

pub struct FractalFlame {
    rng: rand::StdRng,
    pub description: String,
    p: [Real; 2],
    r: f64,
    g: f64,
    b: f64,
}

/// calculates A*x+b, for a 2x2 Matrix A
fn affine_transformation(a: [[Real; 2]; 2], b: [Real; 2], x: [Real; 2]) -> [Real; 2]
{
    let mut out = [0.; 2];
    out[0] = a[0][0] * x[0] + a[0][1] * x[1] + b[0];
    out[1] = a[1][0] * x[0] + a[1][1] * x[1] + b[1];
    out
}

const F1C: RGB = RGB(1., 1., 0.);
const F2C: RGB = RGB(1., 0., 0.);
const F3C: RGB = RGB(0., 1., 0.);
const F4C: RGB = RGB(0., 0., 1.);

fn f1(x: [Real; 2]) -> [Real; 2] {
    let a = [[0., 0.], [0., 0.16]];
    let b = [0., 0.];
    affine_transformation(a, b, x)
}

fn f2(x: [Real; 2]) -> [Real; 2] {
    let a = [[0.85, 0.04], [-0.04, 0.85]];
    let b = [0., 1.6];
    affine_transformation(a, b, x)
}

fn f3(x: [Real; 2]) -> [Real; 2] {
    let a = [[0.2, -0.26], [0.23, 0.22]];
    let b = [0., 1.6];
    affine_transformation(a, b, x)
}

fn f4(x: [Real; 2]) -> [Real; 2] {
    let a = [[-0.15, 0.28], [0.26, 0.24]];
    let b = [0., 0.44];
    affine_transformation(a, b, x)
}

impl Iterator for FractalFlame {
    type Item = ([Real; 2], RGB);

    fn next(&mut self) -> Option<([Real; 2], RGB)> {
        let r = self.rng.gen::<f32>();

        match r {
            x if x < 0.01 => {
                self.p = f1(self.p);
                let RGB(r, g, b) = F1C;
                self.r = (r + self.r)/2.;
                self.g = (g + self.g)/2.;
                self.b = (b + self.b)/2.;
            },
            x if x < 0.86 => {
                self.p = f2(self.p);
                let RGB(r, g, b) = F2C;
                self.r = (r + self.r)/2.;
                self.g = (g + self.g)/2.;
                self.b = (b + self.b)/2.;
            },
            x if x < 0.93 => {
                self.p = f3(self.p);
                let RGB(r, g, b) = F3C;
                self.r = (r + self.r)/2.;
                self.g = (g + self.g)/2.;
                self.b = (b + self.b)/2.;
            },
            _ => {
                self.p = f4(self.p);
                let RGB(r, g, b) = F4C;
                self.r = (r + self.r)/2.;
                self.g = (g + self.g)/2.;
                self.b = (b + self.b)/2.;
            },
        };

        Some((self.p, RGB(self.r, self.g, self.b)))
    }
}

impl ColoredIFS for FractalFlame {
    fn description(&self) -> &str {
        &self.description
    }

    fn get_rng(&mut self) -> &mut rand::StdRng {
        &mut self.rng
    }
}


impl ColoredIFSBuilder {
    pub fn fractal_flame(self) -> FractalFlame {
        let rng: rand::StdRng = match self.seed {
            Some(x) => { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) },
            None => rand::StdRng::new().unwrap()
        };

        let description = "Fractal Flame".to_owned();

        info!("Will render {}", description);

        let p = [0., 0.];
        let r = 0.;
        let g = 0.;
        let b = 0.;

        FractalFlame {
            description,
            rng,
            p,
            r,
            g,
            b
        }
    }
}
