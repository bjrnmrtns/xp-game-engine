use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct CameraController {
    pub move_position: Option<Vec2>,
    pub zoom: i32,
}

#[derive(Debug, Default)]
pub struct PlayerController {
    pub rectangle_select: Option<(Vec3, Vec3)>,
}

#[derive(Bundle)]
pub struct CameraCenter;

#[derive(Default, Debug)]
pub struct SelectionRender;

#[derive(Bundle)]
pub struct Unit;
