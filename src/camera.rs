use nalgebra_glm::*;

pub struct Camera {
    initial_up: Vec3,
    initial_direction: Vec3,
    position: Vec3,
    orientation: Quat,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            initial_up: vec3(0.0, 1.0, 0.0),
            initial_direction: vec3(0.0, 0.0, -1.0),
            position: vec3(0.0, 0.0, 2.0),
            orientation: quat_identity(),
        }
    }

    pub fn get_view(&self) -> Mat4 {
        let up = quat_rotate_vec3(&self.orientation, &self.initial_up);
        let direction = quat_rotate_vec3(&self.orientation, &self.initial_direction);
        look_at(&self.position, &(&self.position + &direction), &up)
    }
}