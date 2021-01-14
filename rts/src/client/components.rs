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
    pub position: Vec2,
    pub velocity: Vec2,
}

impl Unit {
    pub fn new(id: u32, position: Vec2) -> Self {
        Self {
            id,
            selected: false,
            desired_position: None,
            position,
            velocity: Vec2::unit_y(),
        }
    }
}
