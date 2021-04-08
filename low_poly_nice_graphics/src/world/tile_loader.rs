use crate::{
    gltf::{load_gltf, MeshLoadError},
    mesh::Mesh,
    registry::Handle,
    transform::Transform,
    world::{tile_loader, Tile, TileConfiguration, TileType},
};
use glam::{Quat, Vec3};
use image::GenericImageView;
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

pub fn load(mut add_mesh: impl FnMut(Mesh) -> Handle<Mesh>) -> Result<HashMap<Tile, Handle<Mesh>>, TileLoadError> {
    let mut mapping = HashMap::new();
    load_prebaked_tiles(&mut mapping, &mut add_mesh);
    load_gltf(std::fs::read("res/gltf/test.gltf").unwrap().as_slice(), |name, mesh| {
        let handle = add_mesh(mesh);
        add_mapping(&mut mapping, handle, TileType::Test, name);
    })?;
    Ok(mapping)
}

pub struct Tiles {
    grid: Vec<u32>,
    width: usize,
    height: usize,
    tile_mapping: HashMap<Tile, Handle<Mesh>>,
}

impl Tiles {
    pub fn load_tilemap(add_mesh: impl FnMut(Mesh) -> Handle<Mesh>) -> Result<Self, tile_loader::TileLoadError> {
        let tile_mapping = tile_loader::load(add_mesh)?;
        let mut grid = Vec::new();
        let world_image =
            image::load_from_memory(std::fs::read("res/map/world100x100.png").unwrap().as_slice()).unwrap();
        let world_rgba = world_image.as_rgb8().unwrap();
        for p in world_rgba.pixels() {
            if p.0 == [0, 255, 0] {
                grid.push(0);
            } else if p.0 == [0, 0, 255] {
                grid.push(1);
            } else {
                grid.push(255);
            }
        }
        Ok(Self {
            grid,
            width: world_image.width() as usize,
            height: world_image.height() as usize,
            tile_mapping,
        })
    }

    // never calculate for edges
    fn get(&self, x: i32, z: i32) -> u32 {
        let x = (x + self.width as i32 / 2) as usize;
        let z = (z + self.height as i32 / 2) as usize;
        assert!(x < self.width);
        assert!(z < self.height);
        self.grid[x + z * self.width]
    }

    pub fn get_tile_type(&self, x: i32, z: i32) -> (Tile, f32) {
        let value = self.get(x, z);
        if value == 0 {
            return (
                Tile {
                    tile_type: TileType::Grass,
                    configuration: TileConfiguration::NoSides,
                },
                0.0,
            );
        } else if value == 1 {
            let left = self.get(x - 1, z);
            let right = self.get(x + 1, z);
            let up = self.get(x, z - 1);
            let down = self.get(x, z + 1);
            match (left, up, right, down) {
                (0, 0, 0, 0) => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::All,
                    },
                    0.0,
                ),
                (0, 0, 1, 1) => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::Corner,
                    },
                    0.0,
                ),
                (1, 0, 0, 1) => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::Corner,
                    },
                    -std::f32::consts::FRAC_PI_2,
                ),
                (1, 1, 0, 0) => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::Corner,
                    },
                    -std::f32::consts::FRAC_PI_2 * 2.0,
                ),
                (0, 1, 1, 0) => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::Corner,
                    },
                    -std::f32::consts::FRAC_PI_2 * 3.0,
                ),
                (0, 0, 0, 1) => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::USide,
                    },
                    0.0,
                ),
                (1, 0, 0, 0) => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::USide,
                    },
                    -std::f32::consts::FRAC_PI_2,
                ),
                (0, 1, 0, 0) => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::USide,
                    },
                    -std::f32::consts::FRAC_PI_2 * 2.0,
                ),
                (0, 0, 1, 0) => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::USide,
                    },
                    -std::f32::consts::FRAC_PI_2 * 3.0,
                ),
                (0, 1, 0, 1) => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::BothSides,
                    },
                    0.0,
                ),
                (1, 0, 1, 0) => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::BothSides,
                    },
                    -std::f32::consts::FRAC_PI_2,
                ),
                (1, 0, 1, 1) => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::OneSide,
                    },
                    0.0,
                ),
                (1, 1, 0, 1) => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::OneSide,
                    },
                    -std::f32::consts::FRAC_PI_2,
                ),
                (1, 1, 1, 0) => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::OneSide,
                    },
                    -std::f32::consts::FRAC_PI_2 * 2.0,
                ),
                (0, 1, 1, 1) => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::OneSide,
                    },
                    -std::f32::consts::FRAC_PI_2 * 3.0,
                ),
                _ => (
                    Tile {
                        tile_type: TileType::Stone,
                        configuration: TileConfiguration::NoSides,
                    },
                    0.0,
                ),
            }
        } else {
            (
                Tile {
                    tile_type: TileType::Empty,
                    configuration: TileConfiguration::NoSides,
                },
                0.0,
            )
        }
    }

    pub fn spawn_entities(&self, mut add_entity: impl FnMut(Handle<Mesh>, Transform)) {
        let half_width = (self.width / 2 - 1) as i32;
        let half_height = (self.height / 2 - 1) as i32;
        for x in -half_width..half_height {
            for z in -half_height..half_height {
                let (tile, rotation) = self.get_tile_type(x, z);
                add_entity(
                    self.tile_mapping.get(&tile).unwrap().clone(),
                    Transform::from_translation_rotation(
                        Vec3::new(x as f32, 0.0, z as f32),
                        Quat::from_rotation_y(rotation),
                    ),
                );
            }
        }
    }
}
