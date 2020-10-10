use crate::graphics::Drawable;
use nalgebra_glm::{identity, quat_identity, quat_to_mat4, translate, vec3, Mat4, Quat, Vec3};

#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq)]
pub enum Kind {
    Player,
    Static,
}

pub struct Entity {
    pub graphics_handle: Option<usize>,
    pub kind: Kind,
    pub position: Vec3,
    pub orientation: Quat,
    pub velocity: f32,
}

impl Entity {
    pub fn new(kind: Kind) -> Self {
        Self {
            graphics_handle: None,
            kind,
            position: vec3(0.0, 1.0, 0.0),
            orientation: quat_identity(),
            velocity: 3.0,
        }
    }
    pub fn model_matrix(&self) -> Mat4 {
        let translate = translate(&identity(), &self.position);
        let rotate = quat_to_mat4(&self.orientation);
        translate * rotate
    }

    pub fn graphics_handle(&self) -> Option<usize> {
        self.graphics_handle
    }
}
