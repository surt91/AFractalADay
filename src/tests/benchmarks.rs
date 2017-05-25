#![feature(test)]
extern crate test;

use functions::Terms;
use self::test::Bencher;

#[bench]
fn bench_run_all_real(b: &mut Bencher) {
    let a = 1f64;
    for f in Terms::generate_candidates() {
        b.iter(|| f(a));
    }
}

#[bench]
fn bench_run_all_complex(b: &mut Bencher) {
    let a = Complex::new(1f64, 1f64);
    for f in Terms::generate_candidates() {
        b.iter(|| f(a));
    }
}

#[bench]
fn bench_run_all_complex_im0(b: &mut Bencher) {
    let a = Complex::new(1f64, 0f64);
    for f in Terms::generate_candidates() {
        b.iter(|| f(a));
    }
}
