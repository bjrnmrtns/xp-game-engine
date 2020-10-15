use crate::entity;
use std::collections::HashSet;
use std::fs::File;

#[derive(Debug, serde::Deserialize)]
pub struct Model {
    pub name: String,
    pub location: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Entity {
    pub model_name: String,
    pub entity_type: entity::EntityType,
    pub start_position: [f32; 3],
    pub max_velocity: f32,
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    pub models: Vec<Model>,
    pub entities: Vec<Entity>,
}

impl Config {
    pub fn default() -> Self {
        Self {
            models: vec![],
            entities: vec![],
        }
    }
    fn is_valid(&self) -> bool {
        let mut uniq = HashSet::new();
        let model_names_uniq = self.models.iter().all(|x| uniq.insert(x.name.clone()));
        model_names_uniq
    }
    pub fn load_config(path: &str) -> Self {
        match File::open(&path) {
            Ok(f) => {
                let config: Result<Config, ron::Error> = ron::de::from_reader(f);
                match config {
                    Ok(config) => {
                        if config.is_valid() {
                            config
                        } else {
                            Config::default()
                        }
                    }
                    Err(_) => Self::default(),
                }
            }
            Err(_) => Self::default(),
        }
    }
}
