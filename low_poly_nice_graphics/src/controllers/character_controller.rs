use crate::input::{Input, KeyCode};
use glam::Vec3;

#[derive(Default)]
pub struct CharacterController {
    pub rotate: f32,
    pub forward: f32,
}

impl CharacterController {
    pub fn keyboard(&mut self, input_state: &Input<KeyCode>) {
        self.forward = input_state.pressed(KeyCode::W) as u32 as f32 * 1.0
            + input_state.pressed(KeyCode::S) as u32 as f32 * -1.0;
        self.rotate = input_state.pressed(KeyCode::D) as u32 as f32 * 1.0
            + input_state.pressed(KeyCode::A) as u32 as f32 * -1.0;
    }
}
