use bevy::prelude::*;

#[derive(Debug, Default, Properties)]
pub struct CharacterController {
    pub move_forward: Option<f32>,
    pub strafe_right: Option<f32>,
    pub direction_change: f32,
    pub jump: bool,
}

impl CharacterController {
    pub fn new() -> Self {
        Self {
            move_forward: None,
            strafe_right: None,
            direction_change: 0.0,
            jump: false,
        }
    }
}

#[derive(Debug, Default, Properties)]
pub struct CollisionShape;
