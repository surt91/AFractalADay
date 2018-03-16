use serde_json;

use super::FractalFlame;
use fractal::{FractalBuilder, RngType};

use fractal::iterated_function_system::serialize::IteratedFunctionSystemConfig;

impl FractalBuilder
{
    pub fn from_config(self, ffc: IteratedFunctionSystemConfig) -> FractalFlame<RngType> {
        let rng = self.seed_rng();

        let number_of_functions = ffc.probabilities.len();
        let probabilities = ffc.probabilities;
        let colors = ffc.colors;
        let transformations = ffc.transformations;
        let description = ffc.description;
        let variation = ffc.variation;
        let post_transform = ffc.post_transform;
        let final_transform = ffc.final_transform;
        let final_color = ffc.final_color;
        let strict_bounds = ffc.strict_bounds;
        let gamma = ffc.gamma;
        let vibrancy = ffc.vibrancy;

        info!("Will render {}", description);

        debug!("number of functions    : {:?}", number_of_functions);
        debug!("cumulative probabilites: {:?}", probabilities);
        debug!("colors                 : {:?}", colors);
        debug!("affine transformations : {:?}", transformations);
        debug!("Variation              : {:?}", variation);

        FractalFlame {
            rng,
            description,
            number_of_functions,
            probabilities,
            colors,
            transformations,
            variation,
            post_transform,
            final_transform,
            final_color,
            strict_bounds,
            gamma,
            vibrancy
        }
    }

    pub fn from_json(self, json: &str) -> FractalFlame<RngType> {
        let ffc: IteratedFunctionSystemConfig = serde_json::from_str(json).expect("invalid json");
        self.from_config(ffc)
    }
}
