use a_fractal_a_day::fractal::{Ising, Boundary};
use criterion::{Criterion, criterion_group, criterion_main};

use num::Complex;

fn mandelbrot_complex(mut z: Complex<f64>, n: usize)  -> Complex<f64> {
    let start = z;
    for _ in 0..n {
        z = z * z + start;
    }
    z
}

fn mandelbrot_manual(mut x: f64, mut y: f64, n: usize) -> (f64, f64) {
    let start_x = x;
    let start_y = y;
    for _ in 0..n {
        x = x*x - y*y + start_x;
        y = x*y + x*y + start_y;
    }
    (x, y)
}

fn mandelbrot_manual2(mut x: f64, mut y: f64, n: usize) -> (f64, f64) {
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

fn mandelbrot_vec_complex(zs: &[Complex<f64>], n: usize) -> Vec<Complex<f64>> {
    zs.iter().map(|&z| {
            let start = z;
            let mut z = z;
            for _ in 0..n {
                z = z * z + start;
            }
            z
        }).collect()
}

// this should be faster since it can use pipelining and maybe sse
fn mandelbrot_vec_complex2(zs: &[Complex<f64>], n: usize) -> Vec<Complex<f64>> {
    let mut out = zs.to_vec();
    let start = zs.to_vec();
    for _ in 0..n {
        for (i, _) in zs.iter().enumerate() {
            out[i] = out[i] * out[i] + start[i];
        }
    }
    out
}

// this should be faster since it can use pipelining and maybe sse
fn mandelbrot_vec_complex3(zs: &[Complex<f64>], n: usize) -> Vec<Complex<f64>> {
    let mut out = zs.to_vec();
    for _ in 0..n {
        for (i, j) in out.iter_mut().enumerate() {
            *j *= *j;
            *j += zs[i];
        }
    }
    out
}

fn ising_helical(l: usize, sweeps: usize) {
    let mut ising = Ising::new(l as u32, l as u32, Boundary::Helical, 42);
    for _ in 0..sweeps {
        ising.sweep();
    }
}

fn ising_periodic(l: usize, sweeps: usize) {
    let mut ising = Ising::new(l as u32, l as u32, Boundary::Periodic, 42);
    for _ in 0..sweeps {
        ising.sweep();
    }
}


fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("ising helical",
        |b| b.iter(|| ising_helical(42, 42))
    );

    c.bench_function("ising periodic",
        |b| b.iter(|| ising_periodic(42, 42))
    );

    c.bench_function("mandelbrot complex",
        |b| b.iter(|| mandelbrot_complex(Complex::new(1., 1.), 100))
    );

    c.bench_function("mandelbrot manual",
        |b| b.iter(|| mandelbrot_manual(1., 1., 100))
    );

    c.bench_function("mandelbrot manual 2",
        |b| b.iter(|| mandelbrot_manual2(1., 1., 100))
    );

    let input = [Complex::new(1., 1.), Complex::new(1., 0.), Complex::new(0., 1.), Complex::new(0., 0.)];
    c.bench_function("mandelbrot vec complex",
        |b| b.iter(|| mandelbrot_vec_complex(&input, 100))
    );

    c.bench_function("mandelbrot vec complex 2",
        |b| b.iter(|| mandelbrot_vec_complex2(&input, 100))
    );

    c.bench_function("mandelbrot vec complex 3",
        |b| b.iter(|| mandelbrot_vec_complex3(&input, 100))
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
