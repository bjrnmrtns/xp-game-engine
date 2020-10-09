use std::fs::File;

#[derive(Debug, serde::Deserialize)]
pub struct Config {
    clipmap_enabled: bool,
    models: Vec<String>,
}

impl Config {
    pub fn default() -> Self {
        Self {
            clipmap_enabled: false,
            models: vec![],
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
