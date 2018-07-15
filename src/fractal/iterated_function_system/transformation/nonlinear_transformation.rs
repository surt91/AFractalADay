use rand::Rng;
use rand::distributions::Standard;

use fractal::Variation;
use numbers::Real;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonlinearTransformation {
    pub variations: Vec<Variation>,
    pub probabilities: Vec<Real>
}

impl NonlinearTransformation {
    pub fn new(variation: Variation) -> NonlinearTransformation {
        NonlinearTransformation {
            variations: vec![variation],
            probabilities: vec![1.]
        }
    }

    pub fn identity() -> NonlinearTransformation {
        NonlinearTransformation::new(Variation::Linear)
    }

    pub fn random<T>(rng: &mut T) -> NonlinearTransformation
        where T: Rng
    {
        let num = rng.gen_range(1, 10);
        let variations: Vec<Variation> = rng.sample_iter(&Standard).take(num).collect();
        let probabilities: Vec<Real> = rng.sample_iter(&Standard).take(num).collect();
        let sum = probabilities.iter().sum::<Real>();
        let probabilities = probabilities.iter().map(|x| x/sum).collect();

        NonlinearTransformation {
            variations,
            probabilities
        }
    }

    // pub fn name(&self) -> String {
    //     self.variation.name()
    // }

    pub fn transform(&self, r: [Real; 2]) -> [Real; 2] {
        let rs: Vec<[Real; 2]> = self.variations.iter()
                                                .map(|v| v.apply(r))
                                                .collect();

        let mut x = 0.;
        let mut y = 0.;
        for (a, b) in self.probabilities.iter().zip(rs) {
            x += a * b[0];
            y += a * b[1];
        }

        [x, y]
    }
}
