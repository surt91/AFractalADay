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

    fn push(&mut self);
    fn pop(&mut self);

    fn turn_right(&mut self) {
        self.turn(-PI/2.);
    }

    fn turn_left(&mut self) {
        self.turn(PI/2.);
    }
}

#[derive(Clone, Debug)]
struct State {
    position: Point,
    direction: f64,
}

pub struct Canvas {
    paths: Vec<Vec<Point>>,
    state: State,
    stack: Vec<State>,
}

impl Turtle for Canvas {
    fn forward(&mut self, d: f64) {
        self.state.position += Point::step(d, self.state.direction);
        self.paths.last_mut().unwrap().push(self.state.position.clone());
    }

    fn turn(&mut self, a: f64) {
        self.state.direction += a;
    }

    fn push(&mut self) {
        self.stack.push(self.state.clone());
    }

    fn pop(&mut self) {
        self.state = self.stack.pop().unwrap(); // panics for ill defined l systems
        self.paths.push(Vec::new());
        self.paths.last_mut().unwrap().push(self.state.position.clone());
    }
}

impl Canvas {
    pub fn new() -> Canvas {
        let start = Point::new(0., 0.);
        let points = vec![start.clone()];
        let paths = vec![points];
        let state = State {
            position: start,
            direction: 0.0,
        };

        Canvas {
            paths,
            state,
            stack: Vec::new(),
        }
    }

    fn bounds(&self) -> (f64, f64, f64, f64) {
        let mut max_x: f64 = -1./0.; // -inf
        let mut max_y: f64 = -1./0.; // -inf
        let mut min_x: f64 = 1./0.; // inf
        let mut min_y: f64 = 1./0.; // inf

        for p in &self.paths {
            max_x = max_x.max(p.iter().map(|p| p.x).fold(-1./0. /* -inf */, f64::max));
            max_y = max_y.max(p.iter().map(|p| p.y).fold(-1./0. /* -inf */, f64::max));
            min_x = min_x.min(p.iter().map(|p| p.x).fold(1./0. /* inf */, f64::min));
            min_y = min_y.min(p.iter().map(|p| p.y).fold(1./0. /* inf */, f64::min));
        }

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
        let scale = f64::min(x as f64 / w, y as f64 / h);
        let scale_r = 1. / scale;
        let x_offset = x as f64 - w * scale;
        let y_offset = y as f64 - h * scale;
        let min_x = min_x - x_offset / 2. * scale_r;
        let min_y = min_y - y_offset / 2. * scale_r;

        let stroke = cmp::min(cmp::min(x, y) / 1000 + 1, 3);
        let stroke_r = stroke as f64 * scale_r;

        let pixels: Vec<(i32, i32)> = iproduct!(0..y as i32, 0..x as i32).collect();

        let lines = self.paths.iter().map(|p| {p.windows(2)});

        // TODO: sort them in some hierachical structure and discard many at once
        // TODO: maybe a quadtree? or something simple such as cells?
        let rects: Vec<(Point, Point, Point, Point)> = lines
            .map(|path| {
                path.map(|line| {
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
            })
            .flatten()
            .collect();

        pixels.par_iter()
            .map(|&(j, i)| {
                let mut color = vec![255, 255, 255, 0];
                    for (n, &(ref p1, ref p2, ref p3, ref p4)) in rects.iter().enumerate() {
                        let q = Point::new(
                            i as f64 * scale_r + min_x,
                            j as f64 * scale_r + min_y
                        );

                        if q.in_rect(&p1, &p2, &p3, &p4) {
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
