use bevy::prelude::*;

#[derive(Bundle)]
pub struct EmptyBundle;

pub struct CameraCenter;

#[derive(Default, Debug)]
pub struct SelectionRender;

#[derive(Bundle, Default)]
pub struct Unit {
    pub selected: bool,
    pub target_position: Option<Vec2>,
}
