use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct ControlableEntities {
    pub selected: Option<Entity>,
    pub entities: Vec<Entity>,
}
