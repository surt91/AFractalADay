use color::hsv2rgb;

#[test]
fn test_hsv2rgb_red() {
    assert_eq!((1., 0., 0.), hsv2rgb(0., 1., 1.));
}
#[test]
fn test_hsv2rgb_yellow() {
    assert_eq!((1., 1., 0.), hsv2rgb(60./360., 1., 1.));
}
#[test]
fn test_hsv2rgb_brown() {
    assert_eq!((0.36, 0.18, 0.09), hsv2rgb(20./360., 0.75, 0.36));
}
#[test]
fn test_hsv2rgb_darkgreen() {
    assert_eq!((0., 0.5, 0.), hsv2rgb(120./360., 1., 0.5));
}
#[test]
fn test_hsv2rgb_orange() {
    assert_eq!((1., 0.5, 0.), hsv2rgb(30./360., 1., 1.));
}
#[test]
fn test_hsv2rgb_safran() {
    assert_eq!((1., 0.75, 0.), hsv2rgb(45./360., 1., 1.));
}
#[test]
fn test_hsv2rgb_indigo() {
    assert_eq!((0.25, 0., 1.), hsv2rgb(255./360., 1., 1.));
}
