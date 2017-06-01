#![feature(test)]
extern crate test;
extern crate a_fractal_a_day;
extern crate num;

use num::Complex;
use a_fractal_a_day::functions::Terms;
use a_fractal_a_day::numbers::{Cplx, Real};
use a_fractal_a_day::iterated_fractal::iterated_fractal_builder::IteratedFractalBuilder;
use a_fractal_a_day::iterated_fractal::IteratedFractal;
use test::Bencher;

#[bench]
fn bench_run_all_real(b: &mut Bencher) {
    let a = 1. as Real;
    let z = Cplx::new(1. as Real, 1. as Real);
    let t = Terms::new();
    for f in t.candidates_real {
        b.iter(|| (f(a).callable)(z));
    }
}

#[bench]
fn bench_run_all_complex(b: &mut Bencher) {
    let a = Complex::new(1. as Real, 1. as Real);
    let z = Complex::new(1. as Real, 1. as Real);
    let t = Terms::new();
    for f in t.candidates_comp {
        b.iter(|| (f(a).callable)(z));
    }
}

#[bench]
fn bench_run_all_complex_im0(b: &mut Bencher) {
    let a = Complex::new(1. as Real, 0. as Real);
    let z = Complex::new(1. as Real, 1. as Real);
    let t = Terms::new();
    for f in t.candidates_comp {
        b.iter(|| (f(a).callable)(z));
    }
}

#[bench]
fn bench_raster_newton(b: &mut Bencher) {
    let nf = IteratedFractalBuilder::new().seed(4).newton();
    b.iter(|| nf.raster((100, 100), (1e-2, 1e-2), (0., 0.)));
}

#[bench]
fn bench_raster_mandelbrot(b: &mut Bencher) {
    let mb = IteratedFractalBuilder::new().seed(9).mandelbrot();
    b.iter(|| mb.raster((100, 100), (1e-2, 1e-2), (0., 0.)));
}

#[bench]
fn mandelbrot_complex(b: &mut Bencher) {

    fn iteration(mut z: Complex<f64>, n: usize)  -> Complex<f64> {
        let start = z;
        for _ in 0..n {
            z = z * z + start;
        }
        z
    }

    b.iter(|| iteration(Complex::new(1., 1.), 100));
}

#[bench]
fn mandelbrot_manual(b: &mut Bencher) {

    fn iteration(mut x: f64, mut y: f64, n: usize) -> (f64, f64) {
        let start_x = x;
        let start_y = y;
        for _ in 0..n {
            x = x*x - y*y + start_x;
            y = x*y + x*y + start_y;
        }
        (x, y)
    }

    b.iter(|| iteration(1., 1., 100));
}
