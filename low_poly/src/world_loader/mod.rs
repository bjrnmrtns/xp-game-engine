mod loader;
mod world_asset;

use crate::world_loader::loader::WorldAssetLoader;
use bevy::prelude::*;

pub use crate::world_loader::world_asset::WorldAsset;

#[derive(Default)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<WorldAsset>()
            .init_asset_loader::<WorldAssetLoader>();
    }
}
