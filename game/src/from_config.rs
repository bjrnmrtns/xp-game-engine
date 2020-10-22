use crate::{configuration, scene};
use nalgebra_glm::quat_identity;

pub fn create_cameras(config: &[configuration::Camera]) -> scene::Cameras {
    let mut cameras = scene::Cameras::new();
    for c in config {
        match c {
            configuration::Camera::Freelook {
                position,
                direction,
            } => cameras.add(scene::Camera::Freelook {
                position: position.clone().into(),
                direction: direction.clone().into(),
            }),
            configuration::Camera::Follow => cameras.add(scene::Camera::Follow),
        }
    }
    cameras
}
pub fn create_entities(config: &[configuration::Entity]) -> (Vec<(u32, &String)>, scene::Entities) {
    let mut entities = scene::Entities::new();
    let mut entity_model_mapping = Vec::new();
    for e in config {
        match e {
            configuration::Entity::Player {
                model_name,
                start_position,
                max_velocity,
            } => {
                let id = entities.add(scene::Entity::Player {
                    pose: scene::Pose {
                        position: start_position.clone().into(),
                        orientation: quat_identity(),
                    },
                    max_velocity: *max_velocity,
                });
                entity_model_mapping.push((id, model_name));
            }
            configuration::Entity::Static {
                model_name,
                start_position,
            } => {
                let id = entities.add(scene::Entity::Static {
                    pose: scene::Pose {
                        position: start_position.clone().into(),
                        orientation: quat_identity(),
                    },
                });
                entity_model_mapping.push((id, model_name));
            }
        }
    }
    (entity_model_mapping, entities)
}

pub fn create_model_meshes(config: &[configuration::Model]) -> Vec<(String, xp_mesh::mesh::Obj)> {
    let mut named_meshes = Vec::new();
    for m in config {
        named_meshes.push((
            m.name.clone(),
            xp_mesh::mesh::Obj::load(m.location.as_str()).unwrap(),
        ));
    }
    named_meshes
}
