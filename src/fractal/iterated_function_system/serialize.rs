use super::NonlinearTransformation;
use super::Transformation;
use color::RGB;

fn default_bounds() -> bool {
    false
}

fn default_gamma() -> f64 {
    4.0
}

fn default_vibrancy() -> f64 {
    0.5
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IteratedFunctionSystemConfig {
    pub probabilities: Vec<f64>,
    pub colors: Vec<RGB>,
    pub transformations: Vec<Transformation>,
    pub variation: NonlinearTransformation,
    pub description: String,
    #[serde(default = "default_bounds")]
    pub strict_bounds: bool,
    #[serde(default = "default_gamma")]
    pub gamma: f64,
    #[serde(default = "default_vibrancy")]
    pub vibrancy: f64
}
