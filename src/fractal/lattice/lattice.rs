use std::usize;

use serde::{self, Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SquareLattice {
    width: u32,
    height: u32,
}

impl SquareLattice {
    pub fn new(width: u32, height: u32) -> Self {
        SquareLattice {
            width,
            height,
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    fn idx2coord(&self, n: usize) -> (u32, u32) {
        let n = n as u32;
        let x = n % self.width;
        let y = n / self.width;
        (x, y)
    }

    fn coord2idx(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn neighbors(&self, n: usize) -> [usize; 4] {
        let (x, y) = self.idx2coord(n);

        let lx = if x == 0 {self.width - 1} else {x - 1};
        let rx = if x == self.width - 1 {0} else {x + 1};
        let uy = if y == 0 {self.height - 1} else {y - 1};
        let dy = if y == self.height - 1 {0} else {y + 1};
        let l = self.coord2idx(lx, y);
        let r = self.coord2idx(rx, y);
        let u = self.coord2idx(x, uy);
        let d = self.coord2idx(x, dy);

        [l, r, u, d]
    }
}
