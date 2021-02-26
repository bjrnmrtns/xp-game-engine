use crate::input::{Input, KeyCode};
use glam::Vec3;

#[derive(Default)]
pub struct CharacterController {
    pub velocity: Vec3,
}

impl CharacterController {
    pub fn keyboard(&mut self, input_state: &Input<KeyCode>) {
        let z = input_state.pressed(KeyCode::W) as u32 as f32 * 1.0;
        let z = z + input_state.pressed(KeyCode::S) as u32 as f32 * -1.0;
        let x = input_state.pressed(KeyCode::D) as u32 as f32 * 1.0;
        let x = x + input_state.pressed(KeyCode::A) as u32 as f32 * -1.0;
        self.velocity = Vec3::new(x, 0.0, z).normalize();
    }
}
