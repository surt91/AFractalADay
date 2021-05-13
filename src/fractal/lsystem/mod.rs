pub mod rules;
pub mod alphabet;
pub mod generic;
pub mod special;

pub use self::generic::Generic;
pub use self::alphabet::Alphabet;
pub use self::rules::Lrules;

use turtle;

pub trait LSystem : Sync {
    fn description(&self) -> &str;
    fn get_canvas(&self) -> turtle::Canvas;
    fn get_serializable(&self) -> Generic;

    fn render(&mut self, resolution: (u32, u32),
                         _scale: Option<f64>,
                         _center: Option<(f64, f64)>)
        -> (Vec<u8>, bool)
    {
        (self.get_canvas().render(resolution), true)
    }
}
