mod koch_curve;
mod turtle;

pub trait LSystem : Sync {
    fn description(&self) -> &str;
    // fn iterate(&mut self, n: u32);
    fn get_canvas(&self) -> turtle::Canvas;

    fn render(&mut self, resolution: (u32, u32),
                         scale: Option<f64>,
                         center: Option<(f64, f64)>)
        -> (Vec<u8>, bool)
    {
        (self.get_canvas().render(resolution), true)
    }
}