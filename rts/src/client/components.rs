use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct CameraCenterController {
    pub move_position: Option<Vec2>,
}

#[derive(Debug, Default)]
pub struct CameraZoomController {
    pub zoom: Option<f32>,
}

#[derive(Debug)]
pub enum Command1 {
    Create,
    Select,
}

#[derive(Debug)]
pub enum Command2 {
    Move(Option<Vec2>),
}

impl Default for Command2 {
    fn default() -> Self {
        Command2::Move(None)
    }
}

impl Default for Command1 {
    fn default() -> Self {
        Command1::Create
    }
}

#[derive(Debug, Default)]
pub struct PlayerController {
    pub rectangle_select: Option<(Vec3, Vec3)>,
    pub command1: Command1,
    pub command2: Command2,
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
