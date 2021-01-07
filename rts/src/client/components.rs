use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct CameraCenterController {
    pub move_position: Option<Vec2>,
}

#[derive(Debug, Default)]
pub struct CameraZoomController {
    pub zoom: Option<f32>,
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
