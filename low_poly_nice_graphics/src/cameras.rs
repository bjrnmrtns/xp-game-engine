use crate::{renderer::Camera, transform::Transform};
use glam::{Mat4, Quat, Vec3};

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
        self.pos
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

pub struct FollowCamera {
    aspect: f32,
    follow: Transform,
}

impl FollowCamera {
    pub fn new(follow: Transform, aspect: f32) -> Self {
        Self { follow, aspect }
    }

    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn follow(&mut self, follow: Transform) {
        self.follow = follow;
    }
}
impl Camera for FollowCamera {
    fn get_position(&self) -> Vec3 {
        Vec3::new(0.0, 10.0, 10.0)
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
        let transform = Transform {
            translation: Vec3::new(0.0, 10.0, 10.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        };
        self.follow.mul_transform(transform).to_matrix().inverse()
    }
}
