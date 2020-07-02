struct TileGenerator {
    lod_0_grid_pitch: i32,
    resolution: i32,
    lod_levels: i32,
    lod_size: i32,
}

impl TileGenerator {
    pub fn new() -> Self {
        Self {
            lod_0_grid_pitch: 1, // fixed point where 1 means 10 cm
            lod_levels: 2,
            resolution: 64, // amount of grid points in widht/height of a tile
            lod_size: 3, // the amount of tiles in width/height of a lod
        }
    }
    pub fn create(&self, lod_level: i32) -> Vec<[i32; 3]> {
        let grid_pitch = self.lod_0_grid_pitch * (lod_level + 1);
        let grid_size = grid_pitch * self.resolution;
        let mut grid = Vec::new();
        for z in 0..grid_size {
            for x in 0..grid_size {
                grid.push([x * grid_pitch, 0, z * grid_pitch]);
            }
        }
        grid
    }
    pub fn within_tile_of_lod(&self, lod_level: i32, tile_position: [i32; 3]) -> Option<[i32; 3]>{
        let lod_diff = lod_level - tile_position[1];
        if lod_diff < 0 {
            None
        } else if lod_diff == 0 {
            Some(tile_position)
        } else {
            Some([tile_position[0] / self.lod_size, lod_level, tile_position[2] / self.lod_size])
        }
    }
    pub fn which_tiles(&self, tile_position: [i32; 3]) -> Vec<[i32; 3]> {
        let mut tiles = Vec::new();
        tiles
    }
}

fn test_which_tiles() {
    let tiles = TileGenerator::new().which_tiles([0, 0, 0]);
    assert_eq!(tiles.len(), 9);
    assert!(tiles.contains(&[-1, 0, -1]));
    assert!(tiles.contains(&[0, 0, -1]));
    assert!(tiles.contains(&[1, 0, -1]));
    assert!(tiles.contains(&[-1, 0, 0]));
    assert!(tiles.contains(&[0, 0, 0]));
    assert!(tiles.contains(&[1, 0, 0]));
    assert!(tiles.contains(&[-1, 0, 1]));
    assert!(tiles.contains(&[0, 0, 1]));
    assert!(tiles.contains(&[1, 0, 1]));
}

#[test]
fn test_within_tile_of_lod() {
    let tile_gen = TileGenerator::new();
    assert_eq!(tile_gen.within_tile_of_lod(2, [0, 3, 0]), None);
    assert_eq!(tile_gen.within_tile_of_lod(0, [0, 0, 0]), Some([0, 0, 0]));
}


