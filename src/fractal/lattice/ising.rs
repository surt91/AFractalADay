use crate::numbers::Real;
use super::{SquareLattice, Boundary};
use super::LatticeFractal;

use log::info;
use serde::{self, Serialize, Deserialize};

use rand::{Rng, SeedableRng};
use crate::fractal::{FractalBuilder, RngType, default_rng};

impl FractalBuilder {
    pub fn ising(self) -> Ising {
        let (w, h) = self.dimensions.unwrap_or((128, 128));
        Ising::new(w, h, Boundary::Helical, self.seed.unwrap_or(42))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ising {
    n: usize,
    lattice: SquareLattice,
    spins: Vec<i8>,
    t: Real,
    exp_lookup: [Real; 9],

    // cache for Wolff algorithm, reduces number of allocations
    #[serde(skip)]
    cluster_scratch: Vec<u8>,
    #[serde(skip)]
    sweep_id: u8,

    #[serde(skip)]
    #[serde(default = "default_rng")]
    rng: RngType,
    pub description: String,
}

impl Ising {
    pub fn new(w: u32, h: u32, boundary: Boundary, seed: u64) -> Ising {
        let mut rng = RngType::seed_from_u64(seed);
        let n = (w*h) as usize;
        // start random
        let spins: Vec<i8> = (0..n).map(|_| if rng.gen_bool(0.5) {1} else {-1}).collect();
        // start ordered
        // let spins: Vec<i8> = (0..n).map(|_| 1).collect();
        let t = 2.269;
        let description = format!("Ising model on a {}x{} square lattice at T = {}", w, h, t);

        let cluster_scratch = vec![0; n];
        let sweep_id = 1;

        // lookup table for exponential functions: (-de as Real / self.t).exp()
        // for de/2 values of -4, -2, 0, 2, 4
        // array is sparse to use de/2 + 4 directly as an index
        let exp_lookup = [
            (-8 as Real / t).exp(),
            0.,
            (-4 as Real / t).exp(),
            0.,
            1.,
            0.,
            1.,
            0.,
            1.,
        ];

        Ising {
            n,
            lattice: SquareLattice::new(w, h, boundary),
            spins,
            t,
            exp_lookup,
            cluster_scratch,
            sweep_id,
            rng,
            description,
        }
    }

    pub fn sweep(&mut self) -> (i32, usize) {
        let num_flip = self.wolff();
        let de = self.single_spinflip();
        (de, num_flip)
    }

    pub fn single_spinflip(&mut self) -> i32 {
        let mut energy_change = 0;
        for _ in 0..self.n {
            let i = self.rng.gen_range(0, self.n-1);
            let mut de: i32 = self.lattice.neighbors(i)
                .iter()
                .map(|&k| self.spins[k] as i32)
                .sum();

            // calculate exp always
            // de *= 2*self.spins[i] as i32;
            // let p = (-de as Real / self.t).exp()

            // use exp lookup table
            de *= self.spins[i] as i32;
            let p = self.exp_lookup[(-de + 4) as usize];

            if p > self.rng.gen::<Real>() {
                self.spins[i] *= -1;
                energy_change += 2*de;
            }
        }
        energy_change
    }

    pub fn wolff(&mut self) -> usize {
        let mut stack: Vec<usize> = Vec::new();
        let p = 1. - (-2./self.t).exp();
        let seed = self.rng.gen_range(0, self.n-1);

        // use a bitmap to remember which spins are already part of the cluster
        // and reset the bitmap before we run out of range
        self.sweep_id += 1;
        if self.sweep_id == 255 {
            self.cluster_scratch.fill(0);
            self.sweep_id = 1;
        }

        stack.push(seed);
        self.cluster_scratch[seed] = self.sweep_id;
        let mut num_flips = 0;

        while !stack.is_empty() {
            let i = stack.pop().unwrap();
            for j in self.lattice.neighbors(i) {
                if self.spins[j] == self.spins[i] && self.cluster_scratch[j] != self.sweep_id {
                    if p > self.rng.gen::<Real>() {
                        self.cluster_scratch[j] = self.sweep_id;
                        stack.push(j);
                    }
                }
            }
            // we can flip the spins always
            self.spins[i] *= -1;
            num_flips += 1;
        }

        num_flips
    }

    fn magnetization(&self) -> Real {
        self.spins.iter().map(|&i| i as i32).sum::<i32>() as Real / self.n as Real
    }

    fn energy(&self) -> Real {
        let ext_energy = (0..self.n).map(|i|
                self.lattice.neighbors(i).iter()
                    .map(|&j| (self.spins[i] * self.spins[j]) as i32)
                    .sum::<i32>()
            )
            .sum::<i32>();

        let mut ext_energy = -ext_energy as Real;
        ext_energy /= 2.;
        ext_energy /= self.n as Real;

        ext_energy
    }

    fn equilibrate(&mut self) {
        let mut largest_cluster = 0;
        let mut t_eq_try = 100;
        while largest_cluster < self.n / 2 {
            for i in 0..t_eq_try {
                let (de, num_flip) = self.sweep();
                largest_cluster = std::cmp::max(num_flip, largest_cluster);
                info!("{}: de = {}, # {}", i, de, num_flip);
            }
            t_eq_try *= 2;
        }
        self.description = format!("{} (m = {:.2}, E = {:.2})", self.description(), self.magnetization(), self.energy())
    }
}

impl LatticeFractal for Ising {
    fn description(&self) -> &str {
        &self.description
    }

    fn render(&mut self, resolution: (u32, u32),
                         _scale: Option<f64>,
                         _center: Option<(f64, f64)>)
        -> (Vec<u8>, bool)
    {
        assert_eq!(self.lattice.dimensions(), resolution);
        self.equilibrate();
        let buffer: Vec<u8> = self.spins.iter()
            .map(|&s| {
                if s > 0 {
                    vec![0, 0, 0, 255]
                } else {
                    vec![255, 255, 255, 255]
                }
            })
            .flatten()
            .collect();

        (buffer, true)
    }

    fn get_serializable(&self) -> Ising {
        self.clone()
    }
}
