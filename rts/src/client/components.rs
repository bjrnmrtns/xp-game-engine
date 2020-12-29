use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct CameraController {
    pub move_position: Option<Vec2>,
}

#[derive(Debug, Default)]
pub struct PlayerController {
    pub place_object: Option<Vec3>,
}

#[derive(Bundle)]
pub struct CameraCenter;
