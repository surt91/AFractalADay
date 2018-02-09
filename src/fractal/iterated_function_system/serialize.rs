use super::NonlinearTransformation;
use super::Transformation;
use color::RGB;

#[derive(Debug, Serialize, Deserialize)]
pub struct IteratedFunctionSystemConfig {
    pub probabilities: Vec<f64>,
    pub colors: Vec<RGB>,
    pub transformations: Vec<Transformation>,
    pub variation: NonlinearTransformation,
    pub description: String,
    pub strict_bounds: bool,
    pub gamma: f64,
    pub vibrancy: f64
}
