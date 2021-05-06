#[derive(Clone)]
struct Chunk {}

struct Chunks {
    chunks: Vec<Option<Chunk>>,
    chunk_size: usize,
    voxel_size: f32,
    position: Option<[f32; 3]>,
    chunk_position: Option<[i32; 3]>,
}

impl Chunks {
    pub fn new(size: usize, chunk_size: usize, fragment_size: f32) -> Self {
        Self {
            chunks: vec![None; size * size * size],
            chunk_size,
            voxel_size: fragment_size,
            position: None,
            chunk_position: None,
        }
    }

    fn position_to_chunk(&self, position: [f32; 3]) -> [i32; 3] {
        let pos_x = (position[0] as f32 / (self.chunk_size as f32 * self.voxel_size)).floor() as i32;
        let pos_y = (position[1] as f32 / (self.chunk_size as f32 * self.voxel_size)).floor() as i32;
        let pos_z = (position[2] as f32 / (self.chunk_size as f32 * self.voxel_size)).floor() as i32;
        [pos_x, pos_y, pos_z]
    }

    pub fn set_position(&mut self, position: [f32; 3]) -> [i32; 3] {
        self.position = Some(position);
        self.chunk_position = Some(self.position_to_chunk(position));
        self.chunk_position.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::world::chunks::Chunks;

    #[test]
    #[test]
    fn test() {
        let chunk_size = 32;
        let voxel_size = 0.1;
        let world_size = 16;

        let mut world = Chunks::new(world_size, chunk_size, voxel_size);
        let position = [0.0001, 0.0001, 0.0001];
        let chunk_position = world.set_position(position);
        assert_eq!([0, 0, 0], chunk_position);
        let position = [-0.0001, -0.0001, -0.0001];
        let chunk_position = world.set_position(position);
        assert_eq!([-1, -1, -1], chunk_position);
        let position = [3.1999, 3.1999, 3.1999];
        let chunk_position = world.set_position(position);
        assert_eq!([0, 0, 0], chunk_position);
        let position = [3.2001, 3.2001, 3.2001];
        let chunk_position = world.set_position(position);
        assert_eq!([1, 1, 1], chunk_position);
        let position = [-3.1999, -3.1999, -3.1999];
        let chunk_position = world.set_position(position);
        assert_eq!([-1, -1, -1], chunk_position);
        let position = [-3.2001, -3.2001, -3.2001];
        let chunk_position = world.set_position(position);
        assert_eq!([-2, -2, -2], chunk_position);
    }
}
