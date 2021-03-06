#![feature(test)]
extern crate test;
extern crate a_fractal_a_day;
extern crate num;

use num::Complex;
use test::Bencher;

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

#[bench]
fn mandelbrot_manual2(b: &mut Bencher) {

    fn iteration(mut x: f64, mut y: f64, n: usize) -> (f64, f64) {
        let start_x = x;
        let start_y = y;
        for _ in 0..n {
            let xx = x*x;
            let yy = y*y;
            let xy = x*y;
            x = xx - yy + start_x;
            y = 2.*xy + start_y;
        }
        (x, y)
    }

    b.iter(|| iteration(1., 1., 100));
}

#[bench]
fn mandelbrot_vec_complex(b: &mut Bencher) {

    fn iteration(zs: &[Complex<f64>], n: usize) -> Vec<Complex<f64>> {
        zs.iter().map(|&z| {
                let start = z;
                let mut z = z;
                for _ in 0..n {
                    z = z * z + start;
                }
                z
            }).collect()
    }

    let input = [Complex::new(1., 1.), Complex::new(1., 0.), Complex::new(0., 1.), Complex::new(0., 0.)];
    b.iter(|| iteration(&input, 100));
}

#[bench]
fn mandelbrot_vec_complex2(b: &mut Bencher) {
    // this should be faster since it can use pipelining and maybe sse
    fn iteration(zs: &[Complex<f64>], n: usize) -> Vec<Complex<f64>> {
        let mut out = zs.to_vec();
        let start = zs.to_vec();
        for _ in 0..n {
            for (i, _) in zs.iter().enumerate() {
                out[i] = out[i] * out[i] + start[i];
            }
        }
        out
    }

    let input = [Complex::new(1., 1.), Complex::new(1., 0.), Complex::new(0., 1.), Complex::new(0., 0.)];
    b.iter(|| iteration(&input, 100));
}
