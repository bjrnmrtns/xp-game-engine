struct TileGenerator {
    lod_0_grid_pitch: i32,
    resolution: i32,
    lod_levels: i32,
    lod_size: i32,
}

fn to_index_of_lod(index: i32, lod: u32) -> i32 {
    assert!(lod > 0);
    if index < 0 {
        let pos_value_mapped = (index * -1 - 1) / 2_i32.pow(lod);
        return (pos_value_mapped + 1) * -1;
    } else {
        return index / 2_i32.pow(lod);
    }
}

impl TileGenerator {
    pub fn new() -> Self {
        Self {
            lod_0_grid_pitch: 1, // fixed point where 1 means 10 cm
            lod_levels: 2,
            resolution: 64, // amount of grid points in widht/height of a tile
            lod_size: 2, // the amount of tiles in width/height of a lod
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
    pub fn which_tiles(&self, tile_position: [i32; 3]) -> Vec<[i32; 3]> {
        let mut tiles = Vec::new();
        tiles
    }
}

//#[test]
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

/*#[test]
fn test_within_tile_of_lod() {
    let tile_gen = TileGenerator::new();
    assert_eq!(tile_gen.within_tile_of_lod(2, [0, 3, 0]), None);
    assert_eq!(tile_gen.within_tile_of_lod(0, [0, 0, 0]), Some([0, 0, 0]));
    assert_eq!(tile_gen.within_tile_of_lod(1, [1, 0, 0]), Some([0, 1, 0]));
    assert_eq!(tile_gen.within_tile_of_lod(1, [2, 0, 0]), Some([1, 1, 0]));
    assert_eq!(tile_gen.within_tile_of_lod(1, [-1, 0, 0]), Some([-1, 1, 0]));
    assert_eq!(tile_gen.within_tile_of_lod(2, [-4, 0, 0]), Some([-1, 2, 0]));
    assert_eq!(tile_gen.within_tile_of_lod(2, [-5, 0, 0]), Some([-2, 2, 0]));
}
*/

#[test]
fn test_to_index_of_lod1() {
    assert_eq!(to_index_of_lod(0, 1), 0);
    assert_eq!(to_index_of_lod(1, 1), 0);
    assert_eq!(to_index_of_lod(2, 1), 1);
    assert_eq!(to_index_of_lod(3, 1), 1);
    assert_eq!(to_index_of_lod(4, 1), 2);
    assert_eq!(to_index_of_lod(5, 1), 2);
    assert_eq!(to_index_of_lod(6, 1), 3);
    assert_eq!(to_index_of_lod(-3, 1), -2);
    assert_eq!(to_index_of_lod(-2, 1), -1);
    assert_eq!(to_index_of_lod(-1, 1), -1);

}

#[test]
fn test_to_index_of_lod2() {
    assert_eq!(to_index_of_lod(0, 2), 0);
    assert_eq!(to_index_of_lod(1, 2), 0);
    assert_eq!(to_index_of_lod(2, 2), 0);
    assert_eq!(to_index_of_lod(3, 2), 0);
    assert_eq!(to_index_of_lod(4, 2), 1);
    assert_eq!(to_index_of_lod(-5, 2), -2);
    assert_eq!(to_index_of_lod(-4, 2), -1);
    assert_eq!(to_index_of_lod(-3, 2), -1);
    assert_eq!(to_index_of_lod(-2, 2), -1);
    assert_eq!(to_index_of_lod(-1, 2), -1);
}

#[test]
fn test_to_index_of_lod3() {
    assert_eq!(to_index_of_lod(7, 3), 0);
    assert_eq!(to_index_of_lod(8, 3), 1);
    assert_eq!(to_index_of_lod(-8, 3), -1);
    assert_eq!(to_index_of_lod(-9, 3), -2);
}
