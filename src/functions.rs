extern crate rand;
use self::rand::Rng;
extern crate num;
use self::num::complex::Complex;


pub struct Terms {
    candidates: Vec<Box<Fn(Complex<f64>) -> (Box<Fn(Complex<f64>) -> Complex<f64> + Sync>, String)>>
}

impl Terms {
    pub fn new() -> Terms{
        let i = Complex {re: 0., im: 1.};

        let mut candidates: Vec<Box<Fn(Complex<f64>) -> (Box<Fn(Complex<f64>) -> Complex<f64> + Sync>, String)>> = Vec::new();

        candidates.push(Box::new(|a| (Box::new(move |_: Complex<f64>| a ),
                                           format!("{}", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a * x),
                                           format!("{} z", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a * x.powf(2.)),
                                           format!("{} z²", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a * x.powf(3.)),
                                           format!("{} z³", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a * x.powf(4.)),
                                           format!("{} z⁴", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a * x.powf(5.)),
                                           format!("{} z⁵", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a * x.powf(6.)),
                                           format!("{} z⁶", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a * x.powf(7.)),
                                           format!("{} z⁷", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a * x.sin()),
                                           format!("{} sin(z)", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a * x.cos()),
                                           format!("{} cos(z)", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a * x.tan()),
                                           format!("{} tan(z)", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a * x.sinh()),
                                           format!("{} sinh(z)", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a * x.cosh()),
                                           format!("{} cosh(z)", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a * x.atanh()),
                                           format!("{} artanh(z)", a))));
        for b in 2..8i8 {
            candidates.push(Box::new(move |a| (Box::new(move |x: Complex<f64>| a * (x*(b as f64).ln()).exp() ),
                                               format!("{} {}ᶻ", a, b))));
        }
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a * x.exp() ),
                                           format!("{} exp(z)", a))));
        candidates.push(Box::new(|a| (Box::new(move |x: Complex<f64>| a * x.ln() ),
                                           format!("{} ln(z)", a))));

        Terms {candidates: candidates}
    }

    pub fn choice(&mut self, a: Complex<f64>, rng: &mut rand::StdRng) -> (Box<Fn(Complex<f64>) -> Complex<f64> + Sync>, String) {
        let num = self.candidates.len();
        self.candidates.swap_remove(rng.gen_range(0, num as usize))(a)
    }
}
