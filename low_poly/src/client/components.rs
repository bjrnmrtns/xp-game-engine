use bevy::prelude::*;

#[derive(Debug, Default, Properties)]
pub struct EntityController {
    pub move_forward: Option<f32>,
    pub strafe_right: Option<f32>,
    pub direction_change: f32,
    pub jump: bool,
}

impl EntityController {
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
pub struct Player {
    pub position: Vec3,
    pub direction: Vec3,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Default::default(),
            direction: Vec3::unit_z(),
        }
    }
    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }
}
