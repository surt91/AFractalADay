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

    fn bound_rect(&self, points: [&Point; 4]) -> (f64, f64, f64, f64) {
        let max_x = points.iter().map(|p| p.x).fold(-1./0. /* -inf */, f64::max);
        let max_y = points.iter().map(|p| p.y).fold(-1./0. /* -inf */, f64::max);
        let min_x = points.iter().map(|p| p.x).fold(1./0. /* inf */, f64::min);
        let min_y = points.iter().map(|p| p.y).fold(1./0. /* inf */, f64::min);

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
        let w = max_x - min_x;
        let h = max_y - min_y;
        // let scale = f64::min(x as f64 / w, y as f64 / h);
        // let scale_r = 1. / scale;

        let stroke = cmp::min(cmp::min(x, y) / 1000 + 1, 3);
        let stroke_r = stroke as f64 * scale_r;

        let segments = self.paths.iter().map(|p| {p.windows(2)});
        let cell_lateral_count = (segments.len() as f64).sqrt() as usize + 1;
        let cell_w = w / cell_lateral_count as f64;
        let cell_h = h / cell_lateral_count as f64;
        let mut cell_list: Vec<Vec<Vec<usize>>> = vec![vec![Vec::new(); cell_lateral_count+1]; cell_lateral_count+1];
        let mut ctr: usize = 0;

        let mut rects: Vec<(Point, Point, Point, Point)> = Vec::new();
        for path in segments {
            for line in path {
                if let &[ref a, ref b] = line {
                    let bearing = (a.y-b.y).atan2(a.x-b.x);
                    let p1 = a.clone() + Point::step(stroke_r, bearing + PI/2.) + Point::step(stroke_r, bearing);
                    let p2 = a.clone() + Point::step(stroke_r, bearing - PI/2.) + Point::step(stroke_r, bearing);
                    let p3 = b.clone() + Point::step(stroke_r, bearing - PI/2.) - Point::step(stroke_r, bearing);
                    let p4 = b.clone() + Point::step(stroke_r, bearing + PI/2.) - Point::step(stroke_r, bearing);

                    let (min_x_rect, min_y_rect, max_x_rect, max_y_rect) = self.bound_rect([&p1, &p2, &p3, &p4]);
                    let cell_start_x = ((min_x_rect - min_x) / cell_w) as usize;
                    let cell_start_y = ((min_y_rect - min_y) / cell_h) as usize;
                    let cell_stop_x = ((max_x_rect - min_x) / cell_w) as usize;
                    let cell_stop_y = ((max_y_rect - min_y) / cell_h) as usize;

                    for x in cell_start_x..=cell_stop_x {
                        for y in cell_start_y..=cell_stop_y {
                            if x < cell_lateral_count && y < cell_lateral_count {
                                cell_list[x][y].push(ctr.clone());
                            }
                        }
                    }

                    ctr += 1;
                    rects.push((p1, p2, p3, p4));
                }
            }
        }

        let pixels: Vec<(i32, i32)> = iproduct!(0..y as i32, 0..x as i32).collect();
        pixels.iter()
            .map(|&(j, i)| {
                let mut color = vec![255, 255, 255, 0];
                let q = Point::new(
                    i as f64 * scale_r + min_x,
                    j as f64 * scale_r + min_y
                );

                let cell_x = ((q.x - min_x) / cell_w) as usize;
                let cell_y = ((q.y - min_y) / cell_h) as usize;

                if cell_x < cell_lateral_count && cell_y < cell_lateral_count {
                    for &n in cell_list[cell_x][cell_y].iter() {
                        let (ref p1, ref p2, ref p3, ref p4) = rects[n];

                        if q.in_rect(&p1, &p2, &p3, &p4) {
                            let progress = n as f64 / rects.len() as f64;
                            let hsv = color::HSV(progress, 1., 1.);
                            let color::RGB(r, g, b) = hsv.to_rgb();
                            color = vec![(r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8, 255];
                            break
                        }
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
