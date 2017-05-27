#![feature(test)]
extern crate test;
extern crate a_fractal_a_day;
extern crate num;

use num::Complex;
use a_fractal_a_day::functions::{Terms, Cplx, Real};
use a_fractal_a_day::newton_fractal::NewtonFractalBuilder;
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
fn bench_raster(b: &mut Bencher) {
    let nf = NewtonFractalBuilder::new().seed(4).build();
    b.iter(|| nf.raster(100, 100, 1e-2, 1e-2));
}
