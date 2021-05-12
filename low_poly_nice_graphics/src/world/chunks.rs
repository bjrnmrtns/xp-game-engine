use crate::{entity::Entity, registry::Handle};

#[derive(Clone)]
pub struct Chunk {
    pub entity: Handle<Entity>,
    pub just_added: bool,
}

pub struct Diff {
    pub removed: [[std::ops::Range<i32>; 3]; 3],
    pub added: [[std::ops::Range<i32>; 3]; 3],
}

pub struct Chunks {
    chunks: Vec<Option<Chunk>>,
    size: usize,
    chunk_size: usize,
    voxel_size: f32,
    previous_position: Option<[f32; 3]>,
    position: Option<[f32; 3]>,
}

impl Chunks {
    pub fn new(size: usize, chunk_size: usize, voxel_size: f32) -> Self {
        Self {
            chunks: vec![None; size * size * size],
            size,
            chunk_size,
            voxel_size,
            previous_position: None,
            position: None,
        }
    }

    pub fn clear_just_added(&mut self) {
        for chunk in &mut self.chunks {
            if let Some(chunk) = chunk {
                chunk.just_added = false;
            }
        }
    }

    pub fn range_diff_1d_i32(&self, old: i32, new: i32) -> ([i32; 2], [i32; 2]) {
        let extent = (self.size as i32) / 2;
        let old_min = old - extent;
        let old_max = old + extent;
        if old < new {
            let diff = new - old;
            let range_removed = [old_min, old_min + diff];
            let range_added = [old_max, old_max + diff];
            (range_removed, range_added)
        } else {
            let diff = old - new;
            let range_removed = [old_max - diff, old_max];
            let range_added = [old_min - diff, old_min];
            (range_removed, range_added)
        }
    }

    pub fn position_to_chunk(&self, position: [f32; 3]) -> [i32; 3] {
        let pos_x = (position[0] as f32 / (self.chunk_size as f32 * self.voxel_size)).floor() as i32;
        let pos_y = (position[1] as f32 / (self.chunk_size as f32 * self.voxel_size)).floor() as i32;
        let pos_z = (position[2] as f32 / (self.chunk_size as f32 * self.voxel_size)).floor() as i32;
        [pos_x, pos_y, pos_z]
    }

    pub fn range_diff_i32(&self, old: [i32; 3], new: [i32; 3]) -> Diff {
        let (x_removed, x_added) = self.range_diff_1d_i32(old[0], new[0]);
        let (y_removed, y_added) = self.range_diff_1d_i32(old[1], new[1]);
        let (z_removed, z_added) = self.range_diff_1d_i32(old[2], new[2]);
        let extent = (self.size as i32) / 2;
        let x_total_range = new[0] - extent..new[0] + extent;
        let y_total_range = new[1] - extent..new[1] + extent;
        let z_total_range = new[2] - extent..new[2] + extent;
        Diff {
            removed: [
                [x_removed[0]..x_removed[1], y_total_range.clone(), z_total_range.clone()],
                [x_total_range.clone(), y_removed[0]..y_removed[1], z_total_range.clone()],
                [x_total_range.clone(), y_total_range.clone(), z_removed[0]..z_removed[1]],
            ],
            added: [
                [x_added[0]..x_added[1], y_total_range.clone(), z_total_range.clone()],
                [x_total_range.clone(), y_added[0]..y_added[1], z_total_range.clone()],
                [x_total_range.clone(), y_total_range.clone(), z_added[0]..z_added[1]],
            ],
        }
    }

    pub fn range_diff(&self) -> Diff {
        let pos = self.position_to_chunk(self.position.unwrap());
        if let Some(previous_position) = self.previous_position {
            let previous_pos = self.position_to_chunk(previous_position);
            self.range_diff_i32(previous_pos, pos)
        } else {
            let extent = self.size as i32 / 2;
            Diff {
                removed: [[0..0, 0..0, 0..0], [0..0, 0..0, 0..0], [0..0, 0..0, 0..0]],
                added: [
                    [
                        pos[0] - extent..pos[0] + extent,
                        pos[1] - extent..pos[1] + extent,
                        pos[2] - extent..pos[2] + extent,
                    ],
                    [0..0, 0..0, 0..0],
                    [0..0, 0..0, 0..0],
                ],
            }
        }
    }

    pub fn set_chunk(&mut self, chunk_position: [i32; 3], chunk: Option<Chunk>) {
        let max = i32::MAX / (self.size as i32 * 2) * self.size as i32;
        let x = (max + chunk_position[0]) as usize % self.size;
        let y = (max + chunk_position[1]) as usize % self.size;
        let z = (max + chunk_position[2]) as usize % self.size;
        self.chunks[z * self.size * self.size + y * self.size + x] = chunk;
    }

    pub fn get_chunk(&mut self, chunk_position: [i32; 3]) -> Option<Chunk> {
        let max = i32::MAX / (self.size as i32 * 2) * self.size as i32;
        let x = (max + chunk_position[0]) as usize % self.size;
        let y = (max + chunk_position[1]) as usize % self.size;
        let z = (max + chunk_position[2]) as usize % self.size;
        self.chunks[z * self.size * self.size + y * self.size + x].clone()
    }

    pub fn set_position(&mut self, position: [f32; 3]) {
        self.previous_position = self.position;
        self.position = Some(position);
    }
}

#[cfg(test)]
mod tests {
    use crate::world::chunks::Chunks;

    #[test]
    fn position_to_chunk_test() {
        let chunk_size = 32;
        let voxel_size = 0.1;
        let world_size = 16;

        let mut world = Chunks::new(world_size, chunk_size, voxel_size);
        let position = [0.0001, 0.0001, 0.0001];
        let chunk_position = world.position_to_chunk(position);
        assert_eq!([0, 0, 0], chunk_position);
        let position = [-0.0001, -0.0001, -0.0001];
        let chunk_position = world.position_to_chunk(position);
        assert_eq!([-1, -1, -1], chunk_position);
        let position = [3.1999, 3.1999, 3.1999];
        let chunk_position = world.position_to_chunk(position);
        assert_eq!([0, 0, 0], chunk_position);
        let position = [3.2001, 3.2001, 3.2001];
        let chunk_position = world.position_to_chunk(position);
        assert_eq!([1, 1, 1], chunk_position);
        let position = [-3.1999, -3.1999, -3.1999];
        let chunk_position = world.position_to_chunk(position);
        assert_eq!([-1, -1, -1], chunk_position);
        let position = [-3.2001, -3.2001, -3.2001];
        let chunk_position = world.position_to_chunk(position);
        assert_eq!([-2, -2, -2], chunk_position);
    }

    #[test]
    fn range_diff_1d_test() {
        let chunk_size = 4;
        let voxel_size = 0.1;
        let world_size = 16;

        let world = Chunks::new(world_size, chunk_size, voxel_size);
        let (removed, added) = world.range_diff_1d_i32(0, 2);
        assert_eq!(removed, [-2, 0]);
        assert_eq!(added, [2, 4]);
        let (removed, added) = world.range_diff_1d_i32(0, -2);
        assert_eq!(removed, [0, 2]);
        assert_eq!(added, [-4, -2]);
    }

    #[test]
    fn range_diff_i32_test() {
        let chunk_size = 4;
        let voxel_size = 0.1;
        let world_size = 16;

        let mut world = Chunks::new(world_size, chunk_size, voxel_size);
        let diff = world.range_diff_i32([0, 0, 0], [1, 1, 1]);
        let x = 0;
    }

    #[test]
    fn range_diff_test() {
        let chunk_size = 1;
        let voxel_size = 0.1;
        let world_size = 16;

        let mut world = Chunks::new(world_size, chunk_size, voxel_size);
        world.set_position([0.09999, 0.09999, 0.09999]);
        world.set_position([0.1001, 0.1001, 0.1001]);
        let diff = world.range_diff();
        let x = 0;
    }

    #[test]
    fn modulo_test() {
        assert_eq!(((i32::MAX / (6 * 2) * 6) + 6i32) % 6, 0);
        assert_eq!(((i32::MAX / (6 * 2) * 6) + 5i32) % 6, 5);
        assert_eq!(((i32::MAX / (6 * 2) * 6) + 4i32) % 6, 4);
        assert_eq!(((i32::MAX / (6 * 2) * 6) + 3i32) % 6, 3);
        assert_eq!(((i32::MAX / (6 * 2) * 6) + 2i32) % 6, 2);
        assert_eq!(((i32::MAX / (6 * 2) * 6) + 1i32) % 6, 1);
        assert_eq!(((i32::MAX / (6 * 2) * 6) + 0i32) % 6, 0);
        assert_eq!(((i32::MAX / (6 * 2) * 6) - 1i32) % 6, 5);
        assert_eq!(((i32::MAX / (6 * 2) * 6) - 2i32) % 6, 4);
        assert_eq!(((i32::MAX / (6 * 2) * 6) - 3i32) % 6, 3);
        assert_eq!(((i32::MAX / (6 * 2) * 6) - 4i32) % 6, 2);
        assert_eq!(((i32::MAX / (6 * 2) * 6) - 5i32) % 6, 1);
        assert_eq!(((i32::MAX / (6 * 2) * 6) - 6i32) % 6, 0);
        assert_eq!(((i32::MAX / (6 * 2) * 6) - 7i32) % 6, 5);
    }
}
