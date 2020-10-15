use crate::graphics::Drawable;
use nalgebra_glm::{identity, quat_identity, quat_to_mat4, translate, vec3, Mat4, Quat, Vec3};

#[derive(Clone, Debug, serde::Deserialize, Eq, PartialEq)]
pub enum EntityType {
    Player,
    Static,
}

#[derive(Clone)]
pub struct Entity {
    pub id: Option<u32>,
    pub graphics_handle: Option<usize>,
    pub entity_type: EntityType,
    pub position: Vec3,
    pub orientation: Quat,
    pub max_velocity: f32,
}

impl Entity {
    pub fn model_matrix(&self) -> Mat4 {
        let translate = translate(&identity(), &self.position);
        let rotate = quat_to_mat4(&self.orientation);
        translate * rotate
    }

    pub fn graphics_handle(&self) -> Option<usize> {
        self.graphics_handle
    }
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
    pub fn add(&mut self, entity: Entity) -> u32 {
        let mut entity = entity;
        let id = self.generate();
        entity.id = Some(id);
        self.items.push(entity);
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
