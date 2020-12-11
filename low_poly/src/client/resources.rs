use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Default)]
pub struct WorldGrid {
    pub grid: HashMap<(i32, i32, i32), Entity>,
}

pub struct PhysicsSteps {
    pub done: u64,
}

impl PhysicsSteps {
    pub fn new() -> Self {
        Self { done: 0 }
    }
}

#[derive(Default)]
pub struct MeshMap {
    pub handles: HashMap<String, Handle<Mesh>>,
}
