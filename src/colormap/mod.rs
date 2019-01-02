use std::cmp::Ordering;

use rand::Rng;
use rand::seq::SliceRandom;

use crate::color::{RGB,HSV};
mod cividis_dat;
use self::cividis_dat::CIVIDIS;
mod viridis_dat;
use self::viridis_dat::VIRIDIS;
mod inferno_dat;
use self::inferno_dat::INFERNO;
mod twilight_dat;
use self::twilight_dat::TWILIGHT;

/// take a value `x` between 0 and 1 and return a color corresponding to this value
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Colormap {
    map: Vec<(f64, RGB)>,
    name: String,
}

impl Colormap {
    pub fn value(&self, x: &f64) -> RGB {
        let idx = self.map.binary_search_by(|probe| if &probe.0 < x {Ordering::Less} else {Ordering::Greater}).err().unwrap();
        if idx == 0 {
            return self.map[0].1.clone()
        } else if idx == self.map.len() {
            return self.map.last().unwrap().1.clone()
        }
        let before = self.map[idx-1].0;
        let after = self.map[idx].0;

        let p = (x - before) / (after - before);
        RGB::interpolate_weight(&self.map[idx-1].1, &self.map[idx].1, p)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    fn vec_to_map(val: &Vec<RGB>, name: &str) -> Colormap {
        let num = val.len();
        let idx = (0..num).map(|i| i as f64/num as f64).collect::<Vec<f64>>();

        Colormap {
            map: (0..num).map(|i| (idx[i], val[i].clone())).collect::<Vec<(f64, RGB)>>(),
            name: name.to_string()
        }
    }

    pub fn random<T: Rng>(rng: &mut T) -> Colormap {
        let choices = [
            Colormap::viridis,
            Colormap::cividis,
            Colormap::inferno,
            Colormap::twilight,
            Colormap::hsv,
        ];
        choices.choose(rng).unwrap()()
    }

    // colormaps of matplotlib
    // (https://github.com/matplotlib/matplotlib/blob/d4f1f8d0b1b71c97c3f750dfa9c16c1e9ab3261b/lib/matplotlib/_cm_listed.py)
    pub fn viridis() -> Colormap {
        Colormap::vec_to_map(&VIRIDIS, "viridis")
    }
    pub fn cividis() -> Colormap {
        Colormap::vec_to_map(&CIVIDIS, "cividis")
    }
    pub fn inferno() -> Colormap {
        Colormap::vec_to_map(&INFERNO, "inferno")
    }
    pub fn twilight() -> Colormap {
        Colormap::vec_to_map(&TWILIGHT, "twilight")
    }

    pub fn hsv() -> Colormap {
        Colormap {
            map: (0..255).map(|i| (i as f64/256., HSV(i as f64/256., 1., 1.).to_rgb())).collect::<Vec<(f64, RGB)>>(),
            name: "hsv".to_string()
        }
    }
}
