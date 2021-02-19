use nalgebra_glm::{vec3, Mat4, Quat, Vec3};

pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn identity() -> Self {
        Transform {
            translation: vec3(0.0, 0.0, 0.0),
            rotation: Quat::identity(),
            scale: vec3(1.0, 1.0, 1.0),
        }
    }
}
