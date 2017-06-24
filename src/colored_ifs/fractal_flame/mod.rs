mod affine_transformation;
use self::affine_transformation::AffineTransformation;
mod nonlinear_transformation;
use self::nonlinear_transformation::{Variation, NonlinearTransformation};

extern crate std;
extern crate num;
use itertools;

extern crate rand;
use self::rand::Rng;

use numbers::Real;
use colored_ifs::ColoredIFS;
use color::{RGB, HSV};

use super::colored_ifs_builder::ColoredIFSBuilder;

pub struct FractalFlame {
    rng: rand::StdRng,
    pub description: String,
    seed: usize,
    number_of_functions: usize,
    probabilities: Vec<f64>,
    colors: Vec<RGB>,
    affine_transformations: Vec<AffineTransformation>,
    nonlinear_transformation: NonlinearTransformation,
}

pub struct FractalFlameSampler {
    rng: rand::StdRng,
    number_of_functions: usize,
    probabilities: Vec<f64>,
    colors: Vec<RGB>,
    affine_transformations: Vec<AffineTransformation>,
    nonlinear_transformation: NonlinearTransformation,
    p: [Real; 2],
    r: f64,
    g: f64,
    b: f64,
}

impl Iterator for FractalFlameSampler {
    type Item = ([Real; 2], RGB);

    fn next(&mut self) -> Option<([Real; 2], RGB)> {
        let r = self.rng.gen::<f64>();

        let mut index = 0;
        for i in 0..self.number_of_functions {
            if r < self.probabilities[i] {
                index = i;
                break;
            }
        }

        let linear = self.affine_transformations[index].transform(self.p);
        self.p = self.nonlinear_transformation.transform(linear);

        let RGB(r, g, b) = self.colors[index];
        self.r = (r + self.r)/2.;
        self.g = (g + self.g)/2.;
        self.b = (b + self.b)/2.;

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

    fn get_sampler(&mut self) -> FractalFlameSampler {
        self.seed += 1;
        let s: &[_] = &[self.seed];
        let rng = rand::SeedableRng::from_seed(s);

        let p = [0., 0.];
        let r = 0.;
        let g = 0.;
        let b = 0.;

        FractalFlameSampler {
            rng,
            number_of_functions: self.number_of_functions,
            probabilities: self.probabilities.clone(),
            colors: self.colors.clone(),
            affine_transformations: self.affine_transformations.clone(),
            nonlinear_transformation: self.nonlinear_transformation.clone(),
            p,
            r,
            g,
            b,
        }
    }
}


impl ColoredIFSBuilder {
    pub fn fractal_flame(self) -> FractalFlame {
        let mut rng: rand::StdRng = match self.seed {
            Some(x) => { let s: &[_] = &[x]; rand::SeedableRng::from_seed(s) },
            None => rand::StdRng::new().unwrap()
        };

        let seed = match self.seed {
            Some(x) => x,
            None => 1
        };

        let number_of_functions = rng.gen_range(2, 7);

        let prob: Vec<f64> = rng.gen_iter().take(number_of_functions).collect();
        let mut p = 0.;
        let p_norm: f64 = prob.iter().sum();
        let mut probabilities: Vec<f64> = Vec::new();
        for i in prob {
            p += i/p_norm;
            probabilities.push(p);
        }

        let mut colors: Vec<RGB> = Vec::new();
        for _ in 0..number_of_functions {
            let hsv = HSV(rng.gen(), 1., 1.);
            colors.push(hsv.to_rgb());
        }
        let affine_transformations: Vec<AffineTransformation> =
                itertools::repeat_call(|| AffineTransformation::random(&mut rng))
                          .take(number_of_functions)
                          .collect();

        let nonlinear_transformation = NonlinearTransformation::random(&mut rng);

        let description = format!("Fractal Flame: '{}' Variation, {} affine transformations",
                                   nonlinear_transformation.name(),
                                   number_of_functions);

        info!("Will render {}", description);

        println!("{:?}", number_of_functions);
        println!("{:?}", probabilities);
        println!("{:?}", colors);
        println!("{:?}", affine_transformations);
        println!("{:?}", nonlinear_transformation);

        FractalFlame {
            rng,
            description,
            seed,
            number_of_functions,
            probabilities,
            colors,
            affine_transformations,
            nonlinear_transformation
        }
    }
}
