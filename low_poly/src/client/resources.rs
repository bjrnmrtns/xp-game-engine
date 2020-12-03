use crate::world_loader::World;
use bevy::prelude::*;

#[derive(Default)]
pub struct WorldResource {
    pub handle: Handle<World>,
}
