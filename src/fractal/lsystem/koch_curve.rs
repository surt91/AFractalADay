use fractal::FractalBuilder;

use super::LSystem;
use super::turtle::{Turtle, Canvas};

pub struct KochCurve {

}

impl LSystem for KochCurve {
    fn description(&self) -> &str {
        "Koch Curve"
    }

    fn get_canvas(&self) -> Canvas {
        let mut canvas = Canvas::new();

        canvas.forward(1.);
        canvas.forward(1.);
        canvas.turn_left();
        canvas.forward(1.);
        canvas.forward(1.);
        canvas.forward(1.);
        canvas.turn_right();
        canvas.forward(1.);
        canvas.forward(1.);
        canvas.forward(1.);

        canvas
    }
}

impl FractalBuilder
{
    pub fn koch_curve(self) -> KochCurve {
        KochCurve {}
    }
}
