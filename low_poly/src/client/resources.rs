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
