use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct CameraCenterController {
    pub move_position: Option<Vec2>,
}

#[derive(Debug, Default)]
pub struct CameraZoomController {
    pub zoom: Option<f32>,
}

#[derive(Bundle)]
pub struct CameraCenter;

#[derive(Default, Debug)]
pub struct SelectionRender;

#[derive(Bundle, Default)]
pub struct Unit {
    pub selected: bool,
    pub target_position: Option<Vec2>,
}
