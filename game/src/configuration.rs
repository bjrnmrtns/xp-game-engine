use std::fs::File;

#[derive(Debug, serde::Deserialize)]
pub enum EntityKind {
    Player,
    Static,
}

#[derive(Debug, serde::Deserialize)]
pub struct Model {
    name: String,
    location: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Entity {
    kind: EntityKind,
    model: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    models: Vec<Model>,
    entities: Vec<Entity>,
}

impl Config {
    pub fn default() -> Self {
        Self {
            models: vec![],
            entities: vec![],
        }
    }
    pub fn load_config(path: &str) -> Self {
        match File::open(&path) {
            Ok(f) => match ron::de::from_reader(f) {
                Ok(config) => config,
                Err(_) => Self::default(),
            },
            Err(_) => Self::default(),
        }
    }
}
