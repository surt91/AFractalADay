#![feature(closure_to_fn_coercion)]

#[macro_use] extern crate itertools;
mod newton_fractal;
mod twitter;

use newton_fractal::NewtonFractal;

fn main() {
    // let a = NewtonFractal::new(|z| z.powf(4.) + z.sin() + 15.);
    let mut finished = false;
    let mut detail = String::new();
    while ! finished {
        let f = NewtonFractal::random_formula();
        println!("{}", f.1);
        let a = NewtonFractal::new(f.0);

        detail = format!("{} {}", "Newton Fractal", f.1);

        finished = a.render("test.png") > 10000000;
    }

    twitter::tweet_image(&detail, "test.png");
}
