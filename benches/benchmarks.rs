#![feature(test)]
extern crate test;
extern crate a_fractal_a_day;
extern crate num;

use num::Complex;
use a_fractal_a_day::functions::Terms;
use test::Bencher;

#[bench]
fn bench_run_all_real(b: &mut Bencher) {
    let a = 1f64;
    let z = Complex::new(1f64, 1f64);
    let t = Terms::new();
    for f in t.candidates_real {
        b.iter(|| (f(a).callable)(z));
    }
}

#[bench]
fn bench_run_all_complex(b: &mut Bencher) {
    let a = Complex::new(1f64, 1f64);
    let z = Complex::new(1f64, 1f64);
    let t = Terms::new();
    for f in t.candidates_comp {
        b.iter(|| (f(a).callable)(z));
    }
}

#[bench]
fn bench_run_all_complex_im0(b: &mut Bencher) {
    let a = Complex::new(1f64, 0f64);
    let z = Complex::new(1f64, 1f64);
    let t = Terms::new();
    for f in t.candidates_comp {
        b.iter(|| (f(a).callable)(z));
    }
}

#[bench]
fn bench_run_pointer_1(b: &mut Bencher) {
    let a = Complex::new(1f64, 1f64);
    let z = Complex::new(1f64, 1f64);
    let t = Terms::new();
    let f = t.candidates_comp[1](a).callable;
    b.iter(|| f(z));
}

#[bench]
fn bench_run_local_1(b: &mut Bencher) {
    let a = Complex::new(1f64, 1f64);
    let z = Complex::new(1f64, 1f64);
    b.iter(|| a * z);
}
