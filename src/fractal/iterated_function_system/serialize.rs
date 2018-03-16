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

fn default_post_transform() -> Transformation {
    Transformation::identity()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IteratedFunctionSystemConfig {
    pub probabilities: Vec<f64>,
    pub colors: Vec<RGB>,
    pub transformations: Vec<Transformation>,
    pub variation: NonlinearTransformation,
    #[serde(default = "default_post_transform")]
    pub post_transform: Transformation,
    pub description: String,
    #[serde(default = "default_bounds")]
    pub strict_bounds: bool,
    #[serde(default = "default_gamma")]
    pub gamma: f64,
    #[serde(default = "default_vibrancy")]
    pub vibrancy: f64
}
