use bevy::prelude::*;

#[derive(Debug, Default)]
pub struct FreelookCameras {
    selected: Option<usize>,
    cameras: Vec<Entity>,
}

impl FreelookCameras {
    pub fn new() -> Self {
        Self {
            selected: None,
            cameras: vec![],
        }
    }
    pub fn add(&mut self, entity: Entity) {
        self.cameras.push(entity);
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
pub struct FollowCamera {
    pub entity: Option<Entity>,
    pub camera: Option<Entity>,
}

impl FollowCamera {
    pub fn new() -> Self {
        Self {
            entity: None,
            camera: None,
        }
    }
    pub fn set_entity(&mut self, entity: Entity) {
        self.entity = Some(entity);
    }
    pub fn set_follow_camera(&mut self, camera: Entity) {
        self.camera = Some(camera);
    }
}
