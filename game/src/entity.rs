use nalgebra_glm::{identity, quat_identity, quat_to_mat4, translate, vec3, Mat4, Quat, Vec3};

pub struct Entity {
    pub position: Vec3,
    pub orientation: Quat,
    pub velocity: f32,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            position: vec3(0.0, 1.0, 0.0),
            orientation: quat_identity(),
            velocity: 3.0,
        }
    }
    pub fn to_mat4(&self) -> Mat4 {
        let translate = translate(&identity(), &self.position);
        let rotate = quat_to_mat4(&self.orientation);
        translate * rotate
    }
}
