use bevy::prelude::*;

#[derive(Debug, Default, Properties)]
pub struct EntityController {
    pub transform: Transform,
}

impl EntityController {
    pub fn new() -> Self {
        Self {
            transform: Transform::default(),
        }
    }
    pub fn move_(&mut self, transform: Transform) {
        self.transform = transform;
    }
}
