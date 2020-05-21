use nalgebra_glm::*;

pub enum CameraType {
    FreeLook,
    Follow,
}

fn right_vector(direction: &Vec3) -> Vec3 {
    cross(&direction, &vec3(0.0, 1.0, 0.0))
}

pub fn move_(forward: f32, right: f32, position: &Vec3, direction: &Vec3) -> Vec3 {
    position + direction * forward + right_vector(direction) * right
}

pub fn rotate(updown: f32, around: f32, direction: &Vec3) -> Vec3 {
    let temp_direction = &rotate_vec3(direction, around, &vec3(0.0, 1.0, 0.0)).normalize();
    rotate_vec3(temp_direction, updown, &right_vector(direction)).normalize()
}

pub fn view(position: &Vec3, direction: &Vec3) -> Mat4 {
    look_at(&position, &(position + direction), &vec3(0.0, 1.0, 0.0))
}