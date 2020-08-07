use nalgebra_glm::{Vec3, Quat, Mat4, translate, identity, quat_to_mat4, quat_identity, vec3, look_at, vec4_to_vec3, vec4};
use crate::transformation;

pub trait Posable {
    fn pose(&self) -> Mat4;
}

pub trait Followable {
    fn follow(&self) -> Mat4;
}

pub struct Entity
{
    pub position: Vec3,
    pub orientation: Quat,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            position: vec3(0.0, 0.0, 0.0),
            orientation: quat_identity(),
        }
    }
    pub fn move_(&mut self, forward: f32, right: f32) {
        self.position = transformation::move_along_local_axis(&self.position, &self.orientation, forward, right, 0.0);
    }
    pub fn orient(&mut self, around_y: f32) {
        self.orientation = transformation::rotate_around_local_axis(&self.orientation, 0.0, around_y, 0.0);
    }
}

impl Posable for Entity {
    fn pose(&self) -> Mat4 {
        let translate = translate(&identity(), &self.position);
        let rotate = quat_to_mat4(&self.orientation);
        translate * rotate
    }
}

impl Followable for Entity {
    fn follow(&self) -> Mat4 {
        let direction =  vec4_to_vec3(&(quat_to_mat4(&self.orientation) * vec4(0.0, -1.5, -4.0, 1.0)));
        let eye =  &self.position - &direction;
        look_at(&eye, &self.position, &vec3(0.0, 1.0, 0.0))
    }
}
