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
pub struct CharacterController {
    pub move_forward: Option<f32>,
    pub strafe_right: Option<f32>,
    pub rotate_y: f32,
    pub jump: bool,
    pub action_enabled: bool,
    pub action: Action,
}

#[derive(Debug, Default)]
pub struct CameraController {
    pub rotate_x: f32,
}

#[derive(Bundle)]
pub struct CameraPivot;

pub struct ToolCenter;
