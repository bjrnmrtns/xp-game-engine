use crate::terrain::Generator;

pub struct Sine;

impl Generator for Sine {
    fn generate(&self, pos: [f32; 2]) -> f32 {
        (pos[0] / 4.0).sin() + (pos[1] / 4.0).sin()
    }
}
