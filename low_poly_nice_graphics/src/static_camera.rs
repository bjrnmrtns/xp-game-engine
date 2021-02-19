use crate::renderer::Camera;
use glam::{Mat4, Vec3};

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
        Vec3::new(0.0, 0.0, 0.0)
    }

    fn get_projection(&self) -> Mat4 {
        Mat4::perspective_rh(
            45.0 * std::f32::consts::PI * 2.0 / 360.0,
            self.aspect,
            0.1,
            1000.0,
        )
    }

    fn get_view(&self) -> Mat4 {
        Mat4::look_at_rh(self.pos, self.target, Vec3::new(0.0, 1.0, 0.0))
    }
}
