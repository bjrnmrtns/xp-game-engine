use crate::{controllers::CameraController, renderer::Camera, transform::Transform};
use glam::{Mat4, Quat, Vec3, Vec4, Vec4Swizzles};

pub struct StaticCamera {
    pos: Vec3,
    target: Vec3,
    aspect: f32,
}

impl StaticCamera {
    pub fn new(pos: Vec3, target: Vec3, aspect: f32) -> Self {
        Self { pos, target, aspect }
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
        Mat4::perspective_rh(45.0 * std::f32::consts::PI * 2.0 / 360.0, self.aspect, 0.1, 1000.0)
    }

    fn get_view(&self) -> Mat4 {
        Mat4::look_at_rh(self.pos, self.target, Vec3::new(0.0, 1.0, 0.0))
    }
}

pub struct FollowCamera {
    aspect: f32,
    to_follow: Transform,
    follow_angle: f32,
    follow_distance: f32,
}

impl FollowCamera {
    pub fn new(follow: Transform, aspect: f32) -> Self {
        Self {
            to_follow: follow,
            aspect,
            follow_angle: 45.0,
            follow_distance: 3.0,
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
    }

    pub fn follow(&mut self, to_follow: Transform) {
        self.to_follow = to_follow;
    }

    fn get_camera_transform(&self) -> Mat4 {
        let mut rotate_around = self.to_follow.clone();
        rotate_around.rotation *= Quat::from_rotation_x(-self.follow_angle * std::f32::consts::PI * 2.0 / 360.0);

        let transform = Transform {
            translation: Vec3::new(0.0, 0.0, self.follow_distance),
            ..Default::default()
        };
        rotate_around.mul_transform(transform).to_matrix()
    }

    pub fn handle_camera_controller(&mut self, controller: &CameraController) {
        let new_follow_distance = self.follow_distance - controller.zoom;
        self.follow_distance = if new_follow_distance < 1.0 {
            1.0
        } else {
            new_follow_distance
        };
        let new_follow_angle = self.follow_angle + controller.vertical_angle_update;
        self.follow_angle = if new_follow_angle < 0.0 {
            0.0
        } else if new_follow_angle > 90.0 {
            90.0
        } else {
            new_follow_angle
        };
    }
}
impl Camera for FollowCamera {
    fn get_position(&self) -> Vec3 {
        (self.get_camera_transform() * Vec4::new(0.0, 0.0, 0.0, 1.0)).xyz()
    }

    fn get_projection(&self) -> Mat4 {
        Mat4::perspective_rh(45.0 * std::f32::consts::PI * 2.0 / 360.0, self.aspect, 0.1, 1000.0)
    }

    fn get_view(&self) -> Mat4 {
        self.get_camera_transform().inverse()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_angle_distance_calculation() {
        let angle = 90.0 * std::f32::consts::PI * 2.0 / 360.0;
        println!("{}, {}, {}", angle.cos() * 10.0, angle.sin() * 10.0, 0.0);
    }
}
