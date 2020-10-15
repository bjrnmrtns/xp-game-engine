use nalgebra_glm::{identity, quat_to_mat4, translate, Mat4, Quat, Vec3};

pub fn get_roots(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let det = b * b - 4.0 * a * c;
    if det < 0.0 {
        return None;
    }
    let sqrt_det = det.sqrt();
    let r0 = (-b - sqrt_det) / (2.0 * a);
    let r1 = (-b + sqrt_det) / (2.0 * a);
    Some((r0, r1))
}

pub fn model_matrix(position: &Vec3, orientation: &Quat) -> Mat4 {
    let translate = translate(&identity(), &position);
    let rotate = quat_to_mat4(&orientation);
    translate * rotate
}
