use std::ops::{Add, Sub};

#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

fn inside(p1: &Point, p2: &Point, q: &Point) -> bool
{
    return (p2.y - p1.y)*(q.x - p1.x) + (-p2.x + p1.x)*(q.y - p1.y) >= 0.;
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x,
            y
        }
    }

    pub fn step(length: f64, direction: f64) -> Point {
        Point::new(
            length * direction.cos(),
            length * direction.sin()
        )
    }

    pub fn in_rect(&self, p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> bool {
        inside(p1, p2, self) &&
        inside(p2, p3, self) &&
        inside(p3, p4, self) &&
        inside(p4, p1, self)
    }
}

impl Add<Point> for Point {
    type Output = Self;
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Self;
    fn sub(self, rhs: Point) -> Point {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
