extern crate rand;

use self::rand::Rng;
use std::fmt;

use color;
use super::Convergence;

use super::RngType;

pub trait Stylable {
    fn style(&self, &Convergence) -> color::HSV;
    fn style_name(&self) -> &str;
}

#[derive(Clone)]
pub struct Style {
    pub callable: fn(&Convergence, Option<f64>, Option<f64>) -> color::HSV,
    pub readable: String,
}

impl fmt::Debug for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Style: {}", self.readable)
    }
}

impl Style {
    pub fn from_string(s: &str) -> Option<Style> {
        match s {
            "vibrant" => Some(Style::vibrant()),
            "spooky" => Some(Style::spooky()),
            "strong" => Some(Style::strong()),
            "pastell" => Some(Style::pastell()),
            _ => None
        }
    }
    pub fn vibrant() -> Style {
        Style { callable: style_vibrant, readable: "vibrant".to_string() }
    }
    pub fn spooky() -> Style {
        Style { callable: style_spooky, readable: "spooky".to_string() }
    }
    pub fn strong() -> Style {
        Style { callable: style_strong, readable: "strong".to_string() }
    }
    pub fn pastell() -> Style {
        Style { callable: style_pastell, readable: "pastell".to_string() }
    }

    pub fn name(&self) -> String {
        self.readable.to_owned()
    }

    pub fn num() -> usize {
        4
    }
    pub fn index(i: usize) -> Style {
        match i {
            1 => Style::spooky(),
            2 => Style::strong(),
            3 => Style::pastell(),
            0 | _ => Style::vibrant(),
        }
    }

    pub fn random_style(rng: &mut RngType) -> Style {
        Style::index(rng.gen_range(0, Style::num()))
    }
}

impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.readable)
    }
}


fn style_pastell(info: &Convergence, random_color: Option<f64>, random_count: Option<f64>) -> color::HSV {
    let random_color = random_color.unwrap_or(1.);
    let random_count = random_count.unwrap_or(1.);

    let hue = (info.value.norm() as f64 * 10. * random_color).fract();
    let hue = if hue.is_nan() {0.} else {hue};
    let value = 1f64;
    let tmp = info.count as f64 / (10. + 40. * random_count);
    let saturation = 1f64.min(tmp);

    color::HSV(hue, saturation, value)
}

fn style_vibrant(info: &Convergence, random_color: Option<f64>, random_count: Option<f64>) -> color::HSV {
    let random_color = random_color.unwrap_or(1.);
    let random_count = random_count.unwrap_or(1.);

    let hue = (info.value.norm() as f64 * 10. * (random_color + 0.1)).fract();
    let hue = if hue.is_nan() {0.} else {hue};
    let value = 1f64;
    let tmp = info.count as f64 / (10. + 40. * random_count);
    let saturation = 1. - 1f64.min(tmp);

    color::HSV(hue, saturation, value)
}

fn style_strong(info: &Convergence, random_color: Option<f64>, random_count: Option<f64>) -> color::HSV {
    let random_color = random_color.unwrap_or(1.);
    let random_count = random_count.unwrap_or(1.);

    let hue = (info.value.norm() as f64 * 10. * random_color).fract();
    let hue = if hue.is_nan() {0.} else {hue};
    let saturation = 1f64;
    let tmp = info.count as f64 / (10. + 100. * random_count);
    let value = 1f64.min(tmp.powf(0.7));

    color::HSV(hue, saturation, value)
}

fn style_spooky(info: &Convergence, random_color: Option<f64>, random_count: Option<f64>) -> color::HSV {
    let random_color = random_color.unwrap_or(1.);
    let random_count = random_count.unwrap_or(1.);

    let hue = (info.value.norm() as f64 * 10. * random_color).fract();
    let hue = if hue.is_nan() {0.} else {hue};
    let saturation = 1f64;
    let tmp = info.count as f64 / (10. + 50. * random_count);
    let value = 1f64.min(tmp);

    color::HSV(hue, saturation, value)
}
