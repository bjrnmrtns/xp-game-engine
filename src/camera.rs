use nalgebra_glm::*;

pub enum CameraType {
    FreeLook,
    Follow,
}

pub struct FreeLook {
    position: Vec3,
    direction: Vec3,
}
impl FreeLook {
    fn right_vector(&self) -> Vec3 {
        cross(&self.direction, &vec3(0.0, 1.0, 0.0))
    }

    pub fn new(position: Vec3, direction: Vec3) -> FreeLook {
        FreeLook { position, direction, }
    }

    pub fn move_(&mut self, forward: f32, right: f32) {
        self.position = &self.position + &self.direction * forward + self.right_vector() * right;
    }

    pub fn camera_rotate(&mut self, updown: f32, around: f32) {
        let temp_direction = &rotate_vec3(&self.direction, around, &vec3(0.0, 1.0, 0.0)).normalize();
        self.direction = rotate_vec3(&temp_direction, updown, &self.right_vector()).normalize()
    }

    pub fn view(&self) -> Mat4 {
        look_at(&self.position, &(&self.position + &self.direction), &vec3(0.0, 1.0, 0.0))
    }
}

pub fn follow_view_matrix(follow_position: Vec3, follow_orientation: Quat) -> Mat4 {
    let camera_direction =  vec4_to_vec3(&(quat_to_mat4(&follow_orientation) * vec4(0.0, -2.0, -3.0, 1.0)));
    let eye =  &follow_position - &camera_direction;
    look_at(&eye, &follow_position, &vec3(0.0, 1.0, 0.0))
}
