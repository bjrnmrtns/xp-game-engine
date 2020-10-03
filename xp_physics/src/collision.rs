use nalgebra_glm::Vec3;

pub struct Collision {
    pub time_to_collision: f32,
    pub distance_to_collision: f32,
    pub position_of_collision: Vec3,
}
