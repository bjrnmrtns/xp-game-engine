use crate::renderer::Height;
use noise::NoiseFn;

pub struct Terrain {
    noise: noise::Fbm,
}

impl Terrain {
    pub fn new() -> Self {
        Self {
            noise: noise::Fbm::new(),
        }
    }
}

impl Height for Terrain {
    fn height(&self, x: f32, y: f32) -> f32 {
        self.noise.get([x as f64, y as f64]) as f32 * 2.0
    }
}
