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

#[derive(Default)]
pub struct UnitIdGenerator {
    pub last_id: Option<u32>,
}

impl UnitIdGenerator {
    pub fn generate(&mut self) -> u32 {
        if let Some(last_id) = self.last_id {
            self.last_id = Some(last_id + 1);
            self.last_id.unwrap()
        } else {
            self.last_id = Some(0);
            self.last_id.unwrap()
        }
    }
}
