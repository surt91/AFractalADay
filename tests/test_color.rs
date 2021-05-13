use assert_approx_eq::assert_approx_eq;

use a_fractal_a_day::color::{HSV, RGB};

macro_rules! rgb_approx_equal {
    ($a:expr, $b:expr) => (
        let RGB(r1, g1, b1) = $a;
        let RGB(r2, g2, b2) = $b;
        assert_approx_eq!(r1, r2);
        assert_approx_eq!(g1, g2);
        assert_approx_eq!(b1, b2);
    )
}
macro_rules! hsv_approx_equal {
    ($a:expr, $b:expr) => (
        let HSV(h1, s1, v1) = $a;
        let HSV(h2, s2, v2) = $b;
        assert_approx_eq!(h1, h2);
        assert_approx_eq!(s1, s2);
        assert_approx_eq!(v1, v2);
    )
}

#[test]
fn test_hsv2rgb_red() {
    assert_eq!(RGB(1., 0., 0.), HSV(0., 1., 1.).to_rgb());
    assert_ne!(RGB(0., 1., 0.), HSV(0., 1., 1.).to_rgb());
    assert_ne!(RGB(1., 1., 0.), HSV(0., 1., 1.).to_rgb());
}
#[test]
fn test_hsv2rgb_yellow() {
    rgb_approx_equal!(RGB(1., 1., 0.), HSV(60./360., 1., 1.).to_rgb());
    hsv_approx_equal!(RGB(1., 1., 0.).to_hsv(), HSV(60./360., 1., 1.));
}
#[test]
fn test_hsv2rgb_brown() {
    rgb_approx_equal!(RGB(0.36, 0.18, 0.09), HSV(20./360., 0.75, 0.36).to_rgb());
    hsv_approx_equal!(RGB(0.36, 0.18, 0.09).to_hsv(), HSV(20./360., 0.75, 0.36));
}
#[test]
fn test_hsv2rgb_darkgreen() {
    rgb_approx_equal!(RGB(0., 0.5, 0.), HSV(120./360., 1., 0.5).to_rgb());
    hsv_approx_equal!(RGB(0., 0.5, 0.).to_hsv(), HSV(120./360., 1., 0.5));
}
#[test]
fn test_hsv2rgb_orange() {
    rgb_approx_equal!(RGB(1., 0.5, 0.), HSV(30./360., 1., 1.).to_rgb());
    hsv_approx_equal!(RGB(1., 0.5, 0.).to_hsv(), HSV(30./360., 1., 1.));
}
#[test]
fn test_hsv2rgb_safran() {
    rgb_approx_equal!(RGB(1., 0.75, 0.), HSV(45./360., 1., 1.).to_rgb());
    hsv_approx_equal!(RGB(1., 0.75, 0.).to_hsv(), HSV(45./360., 1., 1.));
}
#[test]
fn test_hsv2rgb_indigo() {
    rgb_approx_equal!(RGB(0.25, 0., 1.), HSV(255./360., 1., 1.).to_rgb());
    hsv_approx_equal!(RGB(0.25, 0., 1.).to_hsv(), HSV(255./360., 1., 1.));
}
#[test]
fn test_pingpong() {
    let colors = &[RGB(0.25, 0., 1.), RGB(0.36, 0.18, 0.09), RGB(1., 0.75, 0.), RGB(0., 0.5, 0.)];
    for ping in colors {
        let pong = ping.to_hsv().to_rgb();
        rgb_approx_equal!(*ping, pong);
    }
}
