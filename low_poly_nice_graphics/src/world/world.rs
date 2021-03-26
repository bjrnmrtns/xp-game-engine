use crate::tile::{Tile, TileConfiguration, TileType};
use image::GenericImageView;

pub struct World {
    grid: Vec<u32>,
    width: usize,
    height: usize,
}

impl Default for World {
    fn default() -> Self {
        Self {
            grid: vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            width: 8,
            height: 8,
        }
    }
}

impl World {
    pub fn load() -> Self {
        let mut grid = Vec::new();
        let world_image = image::load_from_memory(std::fs::read("res/map/world.png").unwrap().as_slice()).unwrap();
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
        Self {
            grid,
            width: world_image.width() as usize,
            height: world_image.height() as usize,
        }
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
}
