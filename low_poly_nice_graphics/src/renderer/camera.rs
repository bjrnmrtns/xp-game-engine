use glam::{Mat4, Vec3};

pub trait Camera {
    fn get_position(&self) -> Vec3;
    fn get_projection(&self) -> Mat4;
    fn get_view(&self) -> Mat4;
}
