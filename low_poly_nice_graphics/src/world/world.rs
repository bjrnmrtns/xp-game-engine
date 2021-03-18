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
                0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            width: 8,
            height: 8,
        }
    }
}

pub enum WorldTile {
    GrassTopLeft,
    GrassTopRight,
    GrassBottomLeft,
    GrassBottomRight,
    Stone,
}

/*impl World {
    pub fn get(&self, x: usize, z: usize) -> TileType {}
}
 */
