use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct Cameras {
    selected: Option<usize>,
    cameras: Vec<Entity>,
}

impl Cameras {
    pub fn new() -> Self {
        Self {
            selected: None,
            cameras: vec![],
        }
    }
    pub fn add(&mut self, entity: Entity) {
        self.cameras.push(entity);
        if let None = self.selected {
            self.selected = Some(0);
        }
    }
    pub fn toggle(&mut self) {
        self.selected = match (self.selected, self.cameras.len() > 0) {
            (Some(selected), _) => Some((selected + 1) % self.cameras.len()),
            (None, true) => Some(0),
            _ => None,
        };
    }
    pub fn get_selected(&self) -> Option<Entity> {
        if let Some(selected) = self.selected {
            Some(self.cameras[selected])
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct FollowEntity {
    pub entity: Option<Entity>,
}

impl FollowEntity {
    pub fn new() -> Self {
        Self { entity: None }
    }
    pub fn set_entity(&mut self, entity: Entity) {
        self.entity = Some(entity);
    }
}
