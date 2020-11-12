use bevy::prelude::*;

#[derive(Debug, Default, Properties)]
pub struct CharacterController {
    pub move_forward: Option<f32>,
    pub strafe_right: Option<f32>,
    pub rotate_y: f32,
    pub jump: bool,
}

impl CharacterController {
    pub fn new() -> Self {
        Self {
            move_forward: None,
            strafe_right: None,
            rotate_y: 0.0,
            jump: false,
        }
    }
}

#[derive(Debug, Default, Properties)]
pub struct CameraController {
    pub rotate_x: f32,
}

impl CameraController {
    pub fn new() -> Self {
        Self { rotate_x: 0.0 }
    }
}

#[derive(Bundle)]
pub struct CameraNodeThirdPerson {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
}

#[derive(Debug, Default, Properties)]
pub struct CollisionShape;
