use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct ControllableEntities {
    pub selected: Option<usize>,
    pub entities: Vec<Entity>,
}

impl ControllableEntities {
    pub fn new() -> Self {
        Self {
            selected: None,
            entities: vec![],
        }
    }
    pub fn add(&mut self, entity: Entity) {
        self.entities.push(entity);
        if let None = self.selected {
            self.selected = Some(0);
        }
    }
    pub fn toggle(&mut self) {
        self.selected = match (self.selected, self.entities.len() > 0) {
            (Some(selected), _) => Some((selected + 1) % self.entities.len()),
            (None, true) => Some(0),
            _ => None,
        };
    }
    pub fn get_selected(&self) -> Option<Entity> {
        if let Some(selected) = self.selected {
            Some(self.entities[selected])
        } else {
            None
        }
    }
}
