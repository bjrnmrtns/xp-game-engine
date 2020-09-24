use crate::transformation;
use nalgebra_glm::{identity, quat_identity, quat_to_mat4, translate, vec3, Mat4, Quat, Vec3};

pub enum Collider {
    Sphere { radius: f32 },
}

pub struct Pose {
    pub position: Vec3,
    pub orientation: Quat,
}

impl Pose {
    pub fn to_mat4(&self) -> Mat4 {
        let translate = translate(&identity(), &self.position);
        let rotate = quat_to_mat4(&self.orientation);
        translate * rotate
    }
}

pub struct Entity {
    pub pose: Pose,
    pub collider: Collider,
    pub max_acceleration: f32,
    pub velocity: f32,
    pub target_velocity: f32,
    pub max_direction_acceleration: f32,
    pub target_direction: Option<Vec3>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            pose: Pose {
                position: vec3(0.0, 1.0, 0.0),
                orientation: quat_identity(),
            },
            collider: Collider::Sphere { radius: 1.0 },
            max_acceleration: 3.0,
            velocity: 0.0,
            target_velocity: 0.0,
            max_direction_acceleration: 0.5 * std::f32::consts::PI,
            target_direction: None,
        }
    }
    pub fn move_(&mut self, forward: f32, right: f32) {
        self.pose.position = transformation::move_along_local_axis(
            &self.pose.position,
            &self.pose.orientation,
            forward,
            right,
            0.0,
        );
    }
    pub fn orient(&mut self, around_y: f32) {
        self.pose.orientation =
            transformation::rotate_around_local_axis(&self.pose.orientation, 0.0, around_y, 0.0);
    }
}
