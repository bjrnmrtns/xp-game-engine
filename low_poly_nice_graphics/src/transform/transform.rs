use glam::{Mat4, Quat, Vec3};

#[derive(Clone)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub fn identity() -> Self {
        Transform {
            translation: Vec3::zero(),
            rotation: Quat::identity(),
            scale: Vec3::one(),
        }
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Transform {
            translation,
            ..Default::default()
        }
    }

    pub fn from_rotation(rotation: Quat) -> Self {
        Transform {
            rotation,
            ..Default::default()
        }
    }

    pub fn from_translation_rotation(translation: Vec3, rotation: Quat) -> Self {
        Transform {
            translation,
            rotation,
            ..Default::default()
        }
    }

    pub fn from_scale(scale: Vec3) -> Self {
        Transform {
            scale,
            ..Default::default()
        }
    }

    pub fn from_translation_rotation_scale(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Transform {
            translation,
            rotation,
            scale,
        }
    }

    pub fn mul_transform(&self, transform: Transform) -> Transform {
        Transform {
            translation: self.mul_vec3(transform.translation),
            rotation: self.rotation * transform.rotation,
            scale: self.scale * transform.scale,
        }
    }

    pub fn mul_vec3(&self, mut value: Vec3) -> Vec3 {
        value = self.rotation * value;
        value = self.scale * value;
        value += self.translation;
        value
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::unit_z()
    }

    pub fn to_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}
