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
    Grass,
    Stone,
}

impl World {
    // never calculate for edges
    fn get(&self, x: i32, z: i32) -> u32 {
        let x = (x + self.width as i32 / 2) as usize;
        let z = (z + self.height as i32 / 2) as usize;
        assert!(x < self.width);
        assert!(z < self.height);
        self.grid[x + z * self.width]
    }

    pub fn get_tile_type(&self, x: i32, z: i32) -> WorldTile {
        let value = self.get(x, z);
        if value == 0 {
            return WorldTile::Grass;
        } else {
            let left = self.get(x - 1, z);
            let right = self.get(x + 1, z);
            let up = self.get(x, z - 1);
            let down = self.get(x, z + 1);
        }

        WorldTile::Stone
    }
}
