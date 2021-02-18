use crate::registry::handle::Handle;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Registry<T> {
    pub registry: HashMap<u64, T>,
    last_id: Option<u64>,
}
impl<T> Registry<T> {
    pub fn new() -> Self {
        Registry {
            registry: HashMap::default(),
            last_id: None,
        }
    }

    pub fn add(&mut self, asset: T) -> Handle<T> {
        let id = if let Some(last_id) = self.last_id {
            last_id + 1
        } else {
            0u64
        };
        self.last_id = Some(id);
        self.registry.insert(id, asset);
        Handle::new(id)
    }

    pub fn get(&self, handle: Handle<T>) -> Option<&T> {
        self.registry.get(&handle.id)
    }
}
