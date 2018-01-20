extern crate rand;
extern crate serde_json;

use super::FractalFlame;
use super::IteratedFunctionSystemBuilder;
use super::RngType;

use super::serialize::FractalFlameConfig;

impl IteratedFunctionSystemBuilder
{
    pub fn from_json(self, json: &str) -> FractalFlame<RngType> {
        let rng = self.seed_rng();

        let ffc: FractalFlameConfig = serde_json::from_str(json).expect("invalid json");

        let number_of_functions = ffc.probabilities.len();
        let probabilities = ffc.probabilities;
        let colors = ffc.colors;
        let transformations = ffc.transformations;
        let description = ffc.description;
        let variation = ffc.variation;

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
            strict_bounds: true
        }
    }
}
