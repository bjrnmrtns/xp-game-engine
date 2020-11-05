use bevy::prelude::*;

#[derive(Debug, Default, Properties)]
pub struct Player;


#[derive(Debug, Default)]
pub struct Controller {
    pub forward: f32,
    pub right: f32,
    pub toggle_camera: bool,
}
