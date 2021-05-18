mod lattice;
mod ising;

pub use self::lattice::{SquareLattice, Boundary};
pub use self::ising::Ising;

pub trait LatticeFractal : Sync {
    fn description(&self) -> &str;

    fn render(&mut self, _resolution: (u32, u32),
                         _scale: Option<f64>,
                         _center: Option<(f64, f64)>)
        -> (Vec<u8>, bool)
    {
        // TODO
        unimplemented!()
    }

    fn get_serializable(&self) -> Ising;
}
