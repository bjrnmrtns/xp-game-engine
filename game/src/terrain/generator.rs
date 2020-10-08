pub trait Generator {
    fn generate(&self, pos: [f32; 2]) -> f32;
}
