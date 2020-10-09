use crate::Sphere;
use nalgebra_glm::Vec3;

#[derive(Debug)]
pub struct Response {
    pub sphere: Sphere,
    pub movement: Vec3,
}
