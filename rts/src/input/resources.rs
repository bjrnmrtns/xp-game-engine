use bevy::prelude::*;

#[derive(Default)]
pub struct InputState {
    pub last_selection_begin: Option<Vec2>,
    pub world_mouse_position: Vec2,
}

pub enum CameraViewEvent {
    Zoom(f32),
}

pub enum CommandEvent {
    Create(Vec2),
    Select(Vec2, Vec2),
    Move(Vec2),
}
