use serde_json;

use super::FractalFlame;
use fractal::FractalBuilder;

impl FractalBuilder
{
    pub fn ifs_from_json(json: &str) -> Result<FractalFlame, serde_json::Error> {
        serde_json::from_str(json)
    }
}
