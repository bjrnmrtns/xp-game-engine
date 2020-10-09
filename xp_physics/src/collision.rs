use nalgebra_glm::Vec3;

pub const DISTANCE_EPSILON: f32 = 0.001; // 1 mm

pub struct Collision {
    pub time_to: f32,
    pub distance_to: f32,
    pub position: Vec3,
    pub intersection: Vec3,
}
