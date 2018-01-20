use super::nonlinear_transformation::NonlinearTransformation;

use super::Transformation;
use color::RGB;

#[derive(Debug, Serialize, Deserialize)]
pub struct FractalFlameConfig {
    pub probabilities: Vec<f64>,
    pub colors: Vec<RGB>,
    pub transformations: Vec<Transformation>,
    pub variation: NonlinearTransformation,
    pub description: String,
}
