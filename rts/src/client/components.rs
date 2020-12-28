use bevy::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
    Add,
    Remove,
}

impl Default for Action {
    fn default() -> Self {
        Self::Add
    }
}

#[derive(Debug, Default)]
pub struct Controller {
    pub move_position: Option<Vec2>,
}

#[derive(Bundle)]
pub struct CameraCenter;
