use std::collections::HashSet;
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
    model_name: String,
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
    fn is_valid(&self) -> bool {
        let mut uniq = HashSet::new();
        let model_names_uniq = self.models.iter().all(|x| uniq.insert(x.name.clone()));
        let model_names_found = self
            .entities
            .iter()
            .all(|x| !uniq.insert(x.model_name.clone()));
        model_names_uniq && model_names_found
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
