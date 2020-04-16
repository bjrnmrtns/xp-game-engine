use nalgebra_glm::*;

pub struct Camera {
    direction: Vec3,
    position: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            direction: vec3(0.0, 0.0, -1.0),
            position: vec3(0.0, 0.0, 2.0),
        }
    }

    fn right(&self) -> Vec3 {
        cross(&self.direction, &vec3(0.0, 1.0, 0.0))
    }

    pub fn movement(&mut self, forward: f32, right: f32) {
        self.position = &self.position + (&self.direction * forward) + (&self.right() * right);
    }

    pub fn rotation(&mut self, updown: f32, around: f32) {
        self.direction = rotate_vec3(&self.direction, around, &vec3(0.0, 1.0, 0.0)).normalize();
    }

    pub fn get_view(&self) -> Mat4 {
        look_at(&self.position, &(&self.position + &self.direction), &vec3(0.0, 1.0, 0.0))
    }
}