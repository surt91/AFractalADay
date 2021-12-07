use log::info;

use rand::Rng;

use crate::color::HSV;
use crate::fractal::FractalBuilder;
use crate::histogram::BoundsTypes;
use super::QuadraticMap;

use crate::numbers::Real;

impl FractalBuilder
{
    pub fn quadratic_map(self) -> QuadraticMap {
        let mut rng = self.seed_rng();

        let a = match self.qmaprule {
            Some(s) => QuadraticMap::from_string(&s),
            None => (0..12).map(|_| rng.gen::<Real>() * 2.4 - 1.2).collect(),
        };

        let color = HSV(rng.gen(), 1., 1.).to_rgb();

        let gamma = self.gamma.unwrap_or(4.);
        let vibrancy = self.vibrancy.unwrap_or_else(|| rng.gen());
        let bounds = self.bounds.unwrap_or(BoundsTypes::StrictBounds);

        let description = "Quadratic map".to_string();

        info!("Will render {}", description);

        QuadraticMap {
            rng,
            description,
            a,
            color,
            bounds,
            gamma,
            vibrancy,
        }
    }
}
