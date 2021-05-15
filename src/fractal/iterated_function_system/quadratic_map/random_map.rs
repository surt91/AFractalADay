use log::info;

use rand::Rng;

use crate::color::HSV;
use crate::fractal::FractalBuilder;
use super::QuadraticMap;

use crate::numbers::Real;

impl FractalBuilder
{
    pub fn quadratic_map(self) -> QuadraticMap {
        let mut rng = self.seed_rng();

        let mut a = Vec::new();
        for _ in 0..12 {
            a.push(rng.gen::<Real>() * 2.4 - 1.2);
        }

        // let a = QuadraticMap::from_string("VBWNBDELYHUL");

        let color = HSV(rng.gen(), 1., 1.).to_rgb();

        let gamma = match self.gamma {
            Some(s) => s,
            None => 4.
        };

        let vibrancy = match self.vibrancy {
            Some(s) => s,
            None => rng.gen()
        };

        let strict_bounds = rng.gen();

        let description = format!("Quadratic map");

        info!("Will render {}", description);

        QuadraticMap {
            rng,
            description,
            a,
            color,
            strict_bounds,
            gamma,
            vibrancy,
        }
    }
}
