use crate::generators::Height;
use noise::NoiseFn;

pub struct Noise {
    noise: noise::Fbm,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            noise: noise::Fbm::new(),
        }
    }
}

impl Height for Noise {
    fn height(&self, x: f32, y: f32) -> f32 {
        self.noise.get([x as f64, y as f64]) as f32 * 2.0
    }
}

pub struct SineCosine;

impl Height for SineCosine {
    fn height(&self, x: f32, y: f32) -> f32 {
        x.sin() + y.cos()
    }
}
