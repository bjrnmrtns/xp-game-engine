use crate::{Collision, Sphere, Triangle};
use nalgebra_glm::{vec3, Vec3};

pub fn collision_response(sphere: &Sphere, triangle: &Triangle, collision: &Collision) -> Vec3 {
    vec3(0.0, 0.0, 0.0)
}
