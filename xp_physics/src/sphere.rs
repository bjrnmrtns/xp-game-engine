use nalgebra_glm::Vec3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub c: Vec3,
    pub r: f32,
}

impl Sphere {
    pub fn new(c: Vec3, r: f32) -> Self {
        Self { c, r }
    }
}
