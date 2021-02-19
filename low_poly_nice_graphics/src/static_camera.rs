use crate::renderer::Camera;
use nalgebra_glm::{vec3, Mat4, Vec3};

pub struct StaticCamera {
    pos: Vec3,
    target: Vec3,
    aspect: f32,
}

impl StaticCamera {
    pub fn new(pos: Vec3, target: Vec3, aspect: f32) -> Self {
        Self {
            pos,
            target,
            aspect,
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
}

impl Camera for StaticCamera {
    fn get_position(&self) -> Vec3 {
        vec3(0.0, 0.0, 0.0)
    }

    fn get_projection(&self) -> Mat4 {
        nalgebra_glm::perspective(self.aspect, 45.0, 0.1, 1000.0)
    }

    fn get_view(&self) -> Mat4 {
        nalgebra_glm::look_at(&self.pos, &self.target, &vec3(0.0, 1.0, 0.0))
    }
}
