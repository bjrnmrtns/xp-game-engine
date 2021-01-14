use bevy::prelude::*;

#[derive(Bundle)]
pub struct EmptyBundle;

pub struct CameraCenter;

#[derive(Default, Debug)]
pub struct SelectionRender;

#[derive(Bundle, Clone)]
pub struct Unit {
    pub id: u32,
    pub selected: bool,
    pub desired_position: Option<Vec2>,
}

impl Unit {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            selected: false,
            desired_position: None,
        }
    }
}
