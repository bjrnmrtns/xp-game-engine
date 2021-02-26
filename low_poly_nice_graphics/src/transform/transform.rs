use glam::{Mat4, Quat, Vec3};

pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    initial_forward: Vec3,
}

impl Transform {
    pub fn identity() -> Self {
        Transform {
            translation: Vec3::zero(),
            rotation: Quat::identity(),
            scale: Vec3::one(),
            initial_forward: -Vec3::unit_z(),
        }
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * self.initial_forward
    }

    pub fn set_rotation(&mut self, direction: Vec3) {
        self.rotation = Quat::from_rotation_y(direction.angle_between(self.initial_forward));
    }

    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }
}
