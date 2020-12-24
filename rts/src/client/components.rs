use bevy::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    Add,
    Remove,
}

impl Default for Action {
    fn default() -> Self {
        Self::Add
    }
}

#[derive(Debug, Default)]
pub struct Controller {
    pub move_forward: Option<f32>,
    pub strafe_right: Option<f32>,
    pub action_enabled: bool,
    pub action: Action,
}

#[derive(Bundle)]
pub struct CameraCenter;
