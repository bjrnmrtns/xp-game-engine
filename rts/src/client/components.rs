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
pub enum CommandMode {
    Create,
    Command,
}

impl Default for CommandMode {
    fn default() -> Self {
        CommandMode::Create
    }
}

#[derive(Debug, Default)]
pub struct PlayerController {
    pub rectangle_select: Option<(Vec3, Vec3)>,
    pub command_mode: CommandMode,
}

#[derive(Bundle)]
pub struct CameraCenter;

#[derive(Default, Debug)]
pub struct SelectionRender;

#[derive(Bundle, Default)]
pub struct Unit {
    pub(crate) selected: bool,
}
