use crate::{
    mesh::{Mesh, Vertex},
    registry::{Handle, Registry},
    transform::Transform,
    vox,
    vox::Vox,
};
use glam::Vec3;
use std::collections::HashMap;

struct Descriptor {
    pub u: usize,
    pub v: usize,
    pub w: usize,
    pub step: i32,
    pub normal: [i32; 3],
    pub q: [i32; 3],
}

impl Descriptor {
    pub fn new(u: usize, v: usize, w: usize, step: i32, normal: [i32; 3], q: [i32; 3]) -> Self {
        Self {
            u,
            v,
            w,
            step,
            normal,
            q,
        }
    }
}

struct Mask {
    data: Vec<Option<u8>>,
    size_x: usize,
    size_y: usize,
}

impl Mask {
    pub fn new(size_x: usize, size_y: usize) -> Self {
        Self {
            data: vec![None; size_y * size_x],
            size_x,
            size_y,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color_id: Option<u8>) {
        assert!(x < self.size_x);
        assert!(y < self.size_y);
        self.data[y * self.size_x + x] = color_id;
    }

    pub fn get(&mut self, x: usize, y: usize) -> Option<u8> {
        self.data[y * self.size_x + x]
    }
}

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

    pub fn add(&mut self, handle: Handle<Vox>, position: [i32; 3], registry: &Registry<Vox>) {
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
                            self.entities.len() - 1,
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

    pub fn generate(&mut self, registry: &Registry<Vox>, mut add_mesh: impl FnMut(Mesh, Transform)) {
        for (chunk, (vox_id, source_offset, target_offset, size)) in &self.chunk_entity_map {
            let (handle, _, _) = &self.entities[*vox_id];
            let vox = registry.get(handle).unwrap();
            let mut vox_to_gen = Vox::new(self.chunk_size, self.chunk_size, self.chunk_size);
            for z in 0..size[2] {
                for y in 0..size[1] {
                    for x in 0..size[0] {
                        if let Some(color_id) =
                            vox.get(source_offset[0] + x, source_offset[1] + y, source_offset[2] + z)
                        {
                            let color = vox.get_color(color_id);
                            vox_to_gen.set(
                                target_offset[0] + x,
                                target_offset[1] + y,
                                target_offset[2] + z,
                                color_id,
                                color,
                            );
                        }
                    }
                }
            }
            let mesh = greedy_mesh(vox_to_gen);
            add_mesh(
                mesh,
                Transform::from_translation(Vec3::new(
                    chunk.0 as f32 * self.chunk_size as f32 * 0.1,
                    chunk.1 as f32 * self.chunk_size as f32 * 0.1,
                    chunk.2 as f32 * self.chunk_size as f32 * 0.1,
                )),
            );
        }
    }
}

fn greedy_mesh(vox: vox::Vox) -> Mesh {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let descriptors = [
        Descriptor::new(0, 1, 2, 1, [1, 0, 0], [0, 0, 0]),
        Descriptor::new(0, 1, 2, -1, [-1, 0, 0], [1, 0, 0]),
        Descriptor::new(1, 2, 0, 1, [0, 1, 0], [0, 0, 0]),
        Descriptor::new(1, 2, 0, -1, [0, -1, 0], [1, 0, 0]),
        Descriptor::new(2, 0, 1, 1, [0, 0, 1], [0, 0, 0]),
        Descriptor::new(2, 0, 1, -1, [0, 0, -1], [1, 0, 0]),
    ];

    let vox_size = [vox.x_size, vox.y_size, vox.z_size];

    for d in descriptors.iter() {
        let u = d.u;
        let v = d.v;
        let w = d.w;
        let normal = d.normal;
        let normal_outside = [-(normal[0] as f32), -(normal[1] as f32), -(normal[2] as f32)];

        for slice in 0..vox_size[u] {
            let slice = if d.step == 1 { slice } else { vox_size[u] - (slice + 1) };
            let mut cursor = [0, 0, 0];
            let no_voxel_back = (slice == 0 && d.step == 1) || (slice == vox_size[u] - 1 && d.step != 1);
            cursor[u] = slice;
            let mut mask = Mask::new(vox_size[v], vox_size[w]);
            for cursor_w in 0..vox_size[w] {
                for cursor_v in 0..vox_size[v] {
                    cursor[v] = cursor_v;
                    cursor[w] = cursor_w;
                    let voxel_back = if !no_voxel_back {
                        vox.get(
                            (cursor[0] as i32 - normal[0]) as usize,
                            (cursor[1] as i32 - normal[1]) as usize,
                            (cursor[2] as i32 - normal[2]) as usize,
                        )
                    } else {
                        None
                    };
                    let voxel = vox.get(cursor[0], cursor[1], cursor[2]);
                    let color_id = if voxel_back != None && voxel != None && voxel_back == voxel {
                        None
                    } else {
                        voxel
                    };
                    mask.set(cursor[v], cursor[w], color_id);
                }
            }
            for y in 0..vox_size[w] {
                for x in 0..vox_size[v] {
                    let color_id = mask.get(x, y);
                    if let Some(m) = color_id {
                        let mut width = 1;
                        while x + width < vox_size[v] && mask.get(x + width, y) == color_id {
                            width += 1;
                        }
                        let mut height = 1;
                        let mut done = false;
                        while y + height < vox_size[w] && !done {
                            let mut k = 0;
                            while k < width && !done {
                                if mask.get(x + k, y + height) == color_id {
                                    k += 1;
                                } else {
                                    done = true;
                                }
                            }
                            if !done {
                                height += 1;
                            }
                        }
                        let mut base = [0.0, 0.0, 0.0];
                        base[u] = slice as f32 / 10.0 + d.q[0] as f32 / 10.0;
                        base[v] = x as f32 / 10.0 + d.q[1] as f32 / 10.0;
                        base[w] = y as f32 / 10.0 + d.q[2] as f32 / 10.0;

                        let mut dv = [0.0, 0.0, 0.0];
                        dv[v] = width as f32 / 10.0;
                        let mut dw = [0.0, 0.0, 0.0];
                        dw[w] = height as f32 / 10.0;

                        let color = vox.get_color(m);
                        let count = vertices.len() as u32;
                        vertices.extend_from_slice(&[
                            Vertex::new([base[0], base[1], base[2]], normal_outside, color),
                            Vertex::new(
                                [
                                    base[0] + dv[0] + dw[0],
                                    base[1] + dv[1] + dw[1],
                                    base[2] + dv[2] + dw[2],
                                ],
                                normal_outside,
                                color,
                            ),
                            Vertex::new(
                                [base[0] + dv[0], base[1] + dv[1], base[2] + dv[2]],
                                normal_outside,
                                color,
                            ),
                            Vertex::new(
                                [base[0] + dw[0], base[1] + dw[1], base[2] + dw[2]],
                                normal_outside,
                                color,
                            ),
                        ]);
                        if d.step == 1 {
                            indices.extend_from_slice(&[count, count + 1, count + 2, count, count + 3, count + 1]);
                        } else {
                            indices.extend_from_slice(&[count, count + 2, count + 1, count, count + 1, count + 3]);
                        }
                        for yy in y..y + height {
                            for xx in x..x + width {
                                mask.set(xx, yy, None);
                            }
                        }
                    }
                }
            }
        }
    }
    Mesh {
        vertices,
        indices,
        just_loaded: true,
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
