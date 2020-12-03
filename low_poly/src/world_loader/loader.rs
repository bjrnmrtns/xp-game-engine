use crate::world_loader::world_asset::World;
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    utils::BoxedFuture,
};

#[derive(Default)]
pub struct WorldAssetLoader;

impl AssetLoader for WorldAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let world_asset = ron::de::from_bytes::<World>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(world_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["world"]
    }
}
