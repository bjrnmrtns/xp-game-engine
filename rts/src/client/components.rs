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
    pub min_seperation: f32,
    pub max_cohesion: f32,
    pub radius: f32,
    pub max_speed: f32,
    pub max_force: f32,
    pub position: Vec2,
    pub rotation: f32,
    pub velocity: Vec2,
    pub destination: Option<Vec2>,
    pub forces: Vec2,
}

impl Unit {
    pub fn new(id: u32, position: Vec2) -> Self {
        Self {
            id,
            selected: false,
            min_seperation: 0.25,
            max_cohesion: 4.0,
            radius: 0.5,
            max_speed: 4.0,
            max_force: 5.0,
            position,
            rotation: 0.0,
            velocity: Vec2::new(0.0, -1.0),
            destination: None,
            forces: Vec2::zero(),
        }
    }
}

#[derive(Bundle, Clone)]
pub struct Building {
    pub id: u32,
    pub position: Vec2,
    pub size: f32,
}

impl Building {
    pub fn new(id: u32, position: Vec2, size: f32) -> Self {
        Self { id, position, size }
    }
}
