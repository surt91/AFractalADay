extern crate a_fractal_a_day;

use a_fractal_a_day::color::{HSV, RGB};

#[test]
fn test_hsv2rgb_red() {
    assert_eq!(RGB(1., 0., 0.), HSV(0., 1., 1.).to_rgb());
    assert_ne!(RGB(0., 1., 0.), HSV(0., 1., 1.).to_rgb());
    assert_ne!(RGB(1., 1., 0.), HSV(0., 1., 1.).to_rgb());
}
#[test]
fn test_hsv2rgb_yellow() {
    assert_eq!(RGB(1., 1., 0.), HSV(60./360., 1., 1.).to_rgb());
}
#[test]
fn test_hsv2rgb_brown() {
    assert_eq!(RGB(0.36, 0.18, 0.09), HSV(20./360., 0.75, 0.36).to_rgb());
}
#[test]
fn test_hsv2rgb_darkgreen() {
    assert_eq!(RGB(0., 0.5, 0.), HSV(120./360., 1., 0.5).to_rgb());
}
#[test]
fn test_hsv2rgb_orange() {
    assert_eq!(RGB(1., 0.5, 0.), HSV(30./360., 1., 1.).to_rgb());
}
#[test]
fn test_hsv2rgb_safran() {
    assert_eq!(RGB(1., 0.75, 0.), HSV(45./360., 1., 1.).to_rgb());
}
#[test]
fn test_hsv2rgb_indigo() {
    assert_eq!(RGB(0.25, 0., 1.), HSV(255./360., 1., 1.).to_rgb());
}
