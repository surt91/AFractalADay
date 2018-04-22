mod koch_curve;
mod sierpinski_arrowhead;
mod hilbert_curve;
mod gosper_curve;
mod bush;
mod penrose;

mod turtle;

pub trait LSystem : Sync {
    fn description(&self) -> &str;
    fn get_canvas(&self) -> turtle::Canvas;

    fn render(&mut self, resolution: (u32, u32),
                         _scale: Option<f64>,
                         _center: Option<(f64, f64)>)
        -> (Vec<u8>, bool)
    {
        (self.get_canvas().render(resolution), true)
    }
}
