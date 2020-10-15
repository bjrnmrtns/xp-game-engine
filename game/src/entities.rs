use nalgebra_glm::{Quat, Vec3};

#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq)]
pub enum EntityType {
    Player,
    Static,
}

#[derive(Clone)]
pub struct Entity {
    pub id: u32,
    pub entity_type: EntityType,
    pub position: Vec3,
    pub orientation: Quat,
    pub max_velocity: f32,
}

pub struct Entities {
    items: Vec<Entity>,
    last_id: Option<u32>,
}

impl Entities {
    pub fn new() -> Self {
        Self {
            items: vec![],
            last_id: None,
        }
    }
    pub fn add(
        &mut self,
        entity_type: EntityType,
        position: Vec3,
        orientation: Quat,
        max_velocity: f32,
    ) -> u32 {
        let id = self.generate();
        self.items.push(Entity {
            id,
            entity_type,
            position,
            orientation,
            max_velocity,
        });
        self.last_id = Some(id);
        id
    }
    pub fn update(&mut self, entity: Entity) {
        for item in &mut self.items {
            if entity.entity_type == item.entity_type {
                *item = entity.clone();
            }
        }
    }
    pub fn get_first_entity_with(&self, entity_type: EntityType) -> Option<Entity> {
        for item in &self.items {
            if entity_type == item.entity_type {
                return Some(item.clone());
            }
        }
        None
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn get_entities(&self) -> &[Entity] {
        self.items.as_slice()
    }
    fn generate(&self) -> u32 {
        self.last_id.map(|last| last + 1).unwrap_or(0)
    }
}
