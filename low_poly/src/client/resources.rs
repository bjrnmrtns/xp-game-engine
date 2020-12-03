use crate::world_loader::WorldAsset;
use bevy::prelude::*;

#[derive(Default)]
pub struct WorldResource {
    pub handle: Handle<WorldAsset>,
}
