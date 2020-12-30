use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct CameraController {
    pub move_position: Option<Vec2>,
}

#[derive(Debug, Default)]
pub struct PlayerController {
    pub select: Option<Vec3>,
    pub rectangle_select: Option<(Vec3, Vec3)>,
}

#[derive(Bundle)]
pub struct CameraCenter;

#[derive(Default, Debug)]
pub struct SelectionRender;
