use crate::{
    registry::{Handle, Registry},
    vox::Vox,
};
use std::collections::HashMap;

pub struct World {
    entities: Vec<(Handle<Vox>, [usize; 3], [i32; 3])>,
    chunk_entity_map: HashMap<(i32, i32, i32), (usize, [usize; 3], [usize; 3], [usize; 3])>,
    chunk_size: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            chunk_entity_map: HashMap::new(),
            chunk_size: 32,
        }
    }

    fn chunk_number_and_offset(start: i32, chunk_size: usize) -> (i32, usize) {
        if start >= 0 {
            let chunk_number = start / chunk_size as i32;
            let offset = start as usize % chunk_size;
            (chunk_number, offset)
        } else {
            let chunk_number = (start + 1) / chunk_size as i32 - 1;
            let offset = chunk_size - (-start as usize % chunk_size);
            (chunk_number, offset)
        }
    }

    pub fn add(&mut self, handle: Handle<Vox>, position: [i32; 3], registry: Registry<Vox>) {
        let vox = registry.get(&handle).unwrap();
        self.entities
            .push((handle, [vox.x_size, vox.y_size, vox.z_size], position));
        let x_min = position[0];
        let y_min = position[1];
        let z_min = position[2];
        let mut z_size = vox.z_size;
        let (mut z_number, mut target_z_offset) = World::chunk_number_and_offset(z_min, self.chunk_size);
        let mut source_z_offset = 0;
        while z_size != 0 {
            let z_current_size = std::cmp::min(z_size, self.chunk_size - target_z_offset);
            let mut y_size = vox.y_size;
            let (mut y_number, mut target_y_offset) = World::chunk_number_and_offset(y_min, self.chunk_size);
            let mut source_y_offset = 0;
            while y_size != 0 {
                let y_current_size = std::cmp::min(y_size, self.chunk_size - target_y_offset);
                let mut x_size = vox.x_size;
                let (mut x_number, mut target_x_offset) = World::chunk_number_and_offset(x_min, self.chunk_size);
                let mut source_x_offset = 0;
                while x_size != 0 {
                    let x_current_size = std::cmp::min(x_size, self.chunk_size - target_x_offset);
                    self.chunk_entity_map.insert(
                        (x_number, y_number, z_number),
                        (
                            self.entities.len(),
                            [source_x_offset, source_y_offset, source_z_offset],
                            [target_x_offset, target_y_offset, target_z_offset],
                            [x_current_size, y_current_size, z_current_size],
                        ),
                    );
                    x_number += 1;
                    source_x_offset += x_current_size;
                    target_x_offset = 0;
                    x_size -= x_current_size;
                }
                y_number += 1;
                source_y_offset += y_current_size;
                target_y_offset = 0;
                y_size -= y_current_size;
            }
            z_number += 1;
            source_z_offset += z_current_size;
            target_z_offset = 0;
            z_size -= z_current_size;
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::world::World;

    #[test]
    fn offset_test() {
        assert_eq!(World::chunk_number_and_offset(-5, 32), (-1, 27));
        assert_eq!(World::chunk_number_and_offset(-5, 4), (-2, 3));
        assert_eq!(World::chunk_number_and_offset(2, 4), (0, 2));
        assert_eq!(World::chunk_number_and_offset(5, 4), (1, 1));
    }
}
