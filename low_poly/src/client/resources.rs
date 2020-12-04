use crate::world_loader::WorldAsset;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Default)]
pub struct WorldAssetHandle {
    pub handle: Handle<WorldAsset>,
    pub loaded: bool,
}

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
