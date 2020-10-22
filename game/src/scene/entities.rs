use nalgebra_glm::{quat_identity, Quat, Vec3};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Pose {
    pub position: Vec3,
    pub orientation: Quat,
}

pub enum Entity {
    Player { pose: Pose, max_velocity: f32 },
    Static { pose: Pose },
}

pub struct Entities {
    pub entities: HashMap<u32, Entity>,
    last_id: Option<u32>,
}

impl Entities {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            last_id: None,
        }
    }
    pub fn add(&mut self, entity: Entity) -> u32 {
        let id = self.generate_id();
        self.entities.insert(id, entity);
        self.last_id = Some(id);
        id
    }
    pub fn get_with_id(&mut self, id: u32) -> Option<&mut Entity> {
        self.entities.get_mut(&id)
    }
    pub fn get_player(&mut self) -> Option<&mut Entity> {
        for (id, e) in &mut self.entities {
            match e {
                Entity::Player { pose, max_velocity } => {
                    return Some(e);
                }
                _ => (),
            }
        }
        None
    }
    pub fn len(&self) -> usize {
        self.entities.len()
    }
    fn generate_id(&self) -> u32 {
        self.last_id.map(|last| last + 1).unwrap_or(0)
    }
}
