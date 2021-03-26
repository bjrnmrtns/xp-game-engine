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

fn load_prebaked_tiles(mapping: &mut HashMap<Tile, Handle<Mesh>>, mut add_mesh: impl FnMut(Mesh) -> Handle<Mesh>) {
    let tile = Tile {
        tile_type: TileType::Empty,
        configuration: TileConfiguration::NoSides,
    };
    mapping.insert(tile, add_mesh(Mesh::from(tile)));
    let tile = Tile {
        tile_type: TileType::Grass,
        configuration: TileConfiguration::NoSides,
    };
    mapping.insert(tile, add_mesh(Mesh::from(tile)));
    let tile = Tile {
        tile_type: TileType::Stone,
        configuration: TileConfiguration::NoSides,
    };
    mapping.insert(tile, add_mesh(Mesh::from(tile)));
    let tile = Tile {
        tile_type: TileType::Stone,
        configuration: TileConfiguration::All,
    };
    mapping.insert(tile, add_mesh(Mesh::from(tile)));
    let tile = Tile {
        tile_type: TileType::Stone,
        configuration: TileConfiguration::USide,
    };
    mapping.insert(tile, add_mesh(Mesh::from(tile)));
    let tile = Tile {
        tile_type: TileType::Stone,
        configuration: TileConfiguration::Corner,
    };
    mapping.insert(tile, add_mesh(Mesh::from(tile)));
    let tile = Tile {
        tile_type: TileType::Stone,
        configuration: TileConfiguration::BothSides,
    };
    mapping.insert(tile, add_mesh(Mesh::from(tile)));
    let tile = Tile {
        tile_type: TileType::Stone,
        configuration: TileConfiguration::OneSide,
    };
    mapping.insert(tile, add_mesh(Mesh::from(tile)));
}

fn add_mapping(mapping: &mut HashMap<Tile, Handle<Mesh>>, handle: Handle<Mesh>, tile_type: TileType, name: String) {
    if name == "nosides" {
        mapping.insert(Tile::new(tile_type, TileConfiguration::NoSides), handle);
    } else if name == "oneside" {
        mapping.insert(Tile::new(tile_type, TileConfiguration::OneSide), handle);
    } else if name == "bothsides" {
        mapping.insert(Tile::new(tile_type, TileConfiguration::BothSides), handle);
    } else if name == "corner" {
        mapping.insert(Tile::new(tile_type, TileConfiguration::Corner), handle);
    } else if name == "uside" {
        mapping.insert(Tile::new(tile_type, TileConfiguration::USide), handle);
    } else if name == "all" {
        mapping.insert(Tile::new(tile_type, TileConfiguration::All), handle);
    }
}

pub fn load_tiles(
    mut add_mesh: impl FnMut(Mesh) -> Handle<Mesh>,
) -> Result<HashMap<Tile, Handle<Mesh>>, TileLoadError> {
    let mut mapping = HashMap::new();
    load_prebaked_tiles(&mut mapping, &mut add_mesh);
    load_gltf(std::fs::read("res/gltf/test.gltf").unwrap().as_slice(), |name, mesh| {
        let handle = add_mesh(mesh);
        add_mapping(&mut mapping, handle, TileType::Test, name);
    })?;
    Ok(mapping)
}
