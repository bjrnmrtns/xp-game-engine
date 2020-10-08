use crate::terrain::Generator;
use noise::NoiseFn;

pub struct Fbm {
    noise: noise::Fbm,
}

impl Fbm {
    pub fn new() -> Self {
        Self {
            noise: noise::Fbm::new(),
        }
    }
}

impl Generator for Fbm {
    fn generate(&self, pos: [f32; 2]) -> f32 {
        (self
            .noise
            .get([(pos[0] / 40.0) as f64, (pos[1] / 40.0) as f64])
            * 10.0) as f32
    }
}
