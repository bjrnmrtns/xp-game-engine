use nalgebra_glm::{Vec3, Quat, Mat4, translate, identity, quat_to_mat4, quat_identity, vec3,};
use crate::transformation;

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

pub struct Entity
{
    pub pose: Pose,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            pose: Pose {
                position: vec3(0.0, 0.0, 0.0),
                orientation: quat_identity(),
            }
        }
    }
    pub fn move_(&mut self, forward: f32, right: f32) {
        self.pose.position = transformation::move_along_local_axis(&self.pose.position, &self.pose.orientation, forward, right, 0.0);
    }
    pub fn orient(&mut self, around_y: f32) {
        self.pose.orientation = transformation::rotate_around_local_axis(&self.pose.orientation, 0.0, around_y, 0.0);
    }
}

