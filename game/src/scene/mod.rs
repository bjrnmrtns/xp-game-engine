mod cameras;
mod entities;

pub use cameras::*;
pub use entities::*;
use nalgebra_glm::{look_at, quat_to_mat4, vec3, vec4, vec4_to_vec3, Mat4, Vec3};

pub fn view_on(pose: &entities::Pose) -> (Mat4, Vec3) {
    let direction = vec4_to_vec3(&(quat_to_mat4(&pose.orientation) * vec4(0.0, -1.5, -4.0, 1.0)));
    let eye = &pose.position - &direction;
    (look_at(&eye, &pose.position, &vec3(0.0, 1.0, 0.0)), eye)
}
