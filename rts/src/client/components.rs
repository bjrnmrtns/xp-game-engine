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
    pub max_speed: f32,
    pub max_force: f32,
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
    pub destination: Option<Vec2>,
}

impl Unit {
    pub fn new(id: u32, position: Vec2) -> Self {
        Self {
            id,
            selected: false,
            max_speed: 4.0,
            max_force: 5.0,
            position,
            rotation: 0.0,
            velocity: Vec2::zero(),
            destination: None,
        }
    }
}
