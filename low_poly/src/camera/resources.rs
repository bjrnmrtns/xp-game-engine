use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct SelectableCameras {
    pub selected: usize,
    pub camera_ids: Vec<Entity>,
}
