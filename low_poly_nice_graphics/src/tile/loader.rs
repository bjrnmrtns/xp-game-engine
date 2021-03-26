use crate::{
    gltf::{load_gltf, MeshLoadError},
    mesh::Mesh,
    registry::Handle,
    tile::{Tile, TileConfiguration, TileType},
};
use std::collections::HashMap;

#[derive(Debug)]
pub enum TileLoadError {
    MeshLoadError(MeshLoadError),
}

impl From<MeshLoadError> for TileLoadError {
    fn from(e: MeshLoadError) -> TileLoadError {
        TileLoadError::MeshLoadError(e)
    }
}

pub fn load_tiles(mut add_mesh: impl FnMut(Mesh) -> Handle<Mesh>) -> Result<(), TileLoadError> {
    let mut mapping = HashMap::new();
    load_gltf(std::fs::read("res/gltf/test.gltf").unwrap().as_slice(), |name, mesh| {
        let handle = add_mesh(mesh);
        if name == "nosides" {
            mapping.insert(Tile::new(TileType::Grass, TileConfiguration::NoSides), handle);
        } else if name == "oneside" {
            mapping.insert(Tile::new(TileType::Grass, TileConfiguration::OneSide), handle);
        } else if name == "bothsides" {
            mapping.insert(Tile::new(TileType::Grass, TileConfiguration::BothSides), handle);
        } else if name == "corner" {
            mapping.insert(Tile::new(TileType::Grass, TileConfiguration::Corner), handle);
        } else if name == "uside" {
            mapping.insert(Tile::new(TileType::Grass, TileConfiguration::USide), handle);
        } else if name == "all" {
            mapping.insert(Tile::new(TileType::Grass, TileConfiguration::All), handle);
        }
    })?;
    Ok(())
}
