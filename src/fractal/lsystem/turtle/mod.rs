mod point;
use self::point::Point;

use std::f64::consts::PI;
use std::cmp;

extern crate rayon;
use self::rayon::prelude::*;

use color;

pub trait Turtle {
    fn forward(&mut self, d: f64);
    fn turn(&mut self, a: f64);

    fn turn_right(&mut self) {
        self.turn(-PI/2.);
    }

    fn turn_left(&mut self) {
        self.turn(PI/2.);
    }
}

pub struct Canvas {
    points : Vec<Point>,
    direction: f64,
}

impl Turtle for Canvas {
    fn forward(&mut self, d: f64) {
        let head = self.points.last().unwrap().clone();
        let next = Point::step(d, self.direction) + head;
        self.points.push(next);
    }

    fn turn(&mut self, a: f64) {
        self.direction += a;
    }
}

impl Canvas {
    pub fn new() -> Canvas {
        let points = vec![Point::new(0., 0.); 1];

        Canvas {
            points,
            direction: 0.,
        }
    }

    fn bounds(&self) -> (f64, f64, f64, f64) {
        let mut max_x = self.points.iter().map(|p| p.x).fold(-1./0. /* -inf */, f64::max);
        let mut max_y = self.points.iter().map(|p| p.y).fold(-1./0. /* -inf */, f64::max);
        let mut min_x = self.points.iter().map(|p| p.x).fold(1./0. /* inf */, f64::min);
        let mut min_y = self.points.iter().map(|p| p.y).fold(1./0. /* inf */, f64::min);

        let w = max_x - min_x;
        max_x += 0.1*w;
        min_x -= 0.1*w;

        let h = max_y - min_y;
        max_y += 0.1*h;
        min_y -= 0.1*h;

        (min_x, min_y, max_x, max_y)
    }

    pub fn render(&self, resolution: (u32, u32)) -> Vec<u8> {
        let x = resolution.0;
        let y = resolution.1;
        let (min_x, min_y, max_x, max_y) = self.bounds();

        let w = max_x - min_x;
        let h = max_y - min_y;
        let scale = x as f64 / w;
        let scale_r = 1. / scale;

        // let y = (h * scale) as i32;

        let points = &self.points;

        let stroke = cmp::min(x, y) / 1000 + 1;
        let stroke_r = stroke as f64 * scale_r;

        let rects: Vec<(Point, Point, Point, Point)> = points.windows(2)
            .map(|line| {
                if let &[ref a, ref b] = line {
                    let bearing = (a.y-b.y).atan2(a.x-b.x);
                    let p1 = a.clone() + Point::step(stroke_r, bearing + PI/2.) + Point::step(stroke_r, bearing);
                    let p2 = a.clone() + Point::step(stroke_r, bearing - PI/2.) + Point::step(stroke_r, bearing);
                    let p3 = b.clone() + Point::step(stroke_r, bearing - PI/2.) - Point::step(stroke_r, bearing);
                    let p4 = b.clone() + Point::step(stroke_r, bearing + PI/2.) - Point::step(stroke_r, bearing);
                    (p1, p2, p3, p4)
                } else {
                    let p = Point::new(0., 0.);
                    (p.clone(), p.clone(), p.clone(), p.clone())
                }
            })
            .collect();

        let pixels: Vec<(i32, i32)> = iproduct!(0..y as i32, 0..x as i32).collect();
        pixels.par_iter()
            .map(|&(j, i)| {
                let mut color = vec![255, 255, 255, 255];
                    for (n, &(ref p1, ref p2, ref p3, ref p4)) in rects.iter().enumerate() {
                        let q = Point::new(
                            i as f64 * scale_r + min_x,
                            j as f64 * scale_r + min_y
                        );

                        if q.in_rect(&p1, &p2, &p3, &p4) {
                            // TODO color by length?
                            let progress = n as f64 / rects.len() as f64;
                            let hsv = color::HSV(progress, 1., 1.);
                            let color::RGB(r, g, b) = hsv.to_rgb();
                            color = vec![(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8, 255];
                            break
                        }
                    }
                color
              })
              .flatten()
              .collect()
    }

    // fn ascii(&self) -> String {
    //
    // }

    // fn svg(&self) -> String {
    //
    // }
}
