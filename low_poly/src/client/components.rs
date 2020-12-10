use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct CharacterController {
    pub move_forward: Option<f32>,
    pub strafe_right: Option<f32>,
    pub rotate_y: f32,
    pub jump: bool,
    pub place_object: bool,
}

impl CharacterController {
    pub fn new() -> Self {
        Self {
            move_forward: None,
            strafe_right: None,
            rotate_y: 0.0,
            jump: false,
            place_object: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct CameraController {
    pub rotate_x: f32,
}

impl CameraController {
    pub fn new() -> Self {
        Self { rotate_x: 0.0 }
    }
}

#[derive(Bundle)]
pub struct CameraPivot;

pub struct ToolCenter;
