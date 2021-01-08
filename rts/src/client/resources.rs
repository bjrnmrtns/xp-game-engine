use bevy::prelude::*;

#[derive(Default)]
pub struct PhysicsState {
    pub steps_done: u64,
}

#[derive(Default)]
pub struct GameInfo {
    pub camera: Option<Entity>,
    pub camera_center: Option<Entity>,
}
