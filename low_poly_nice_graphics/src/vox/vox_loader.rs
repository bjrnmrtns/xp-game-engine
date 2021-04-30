use crate::{
    mesh::{Cube, Mesh, Vertex},
    registry::{Handle, Registry},
};
use std::collections::{HashMap, HashSet};

fn front_face(pos: [f32; 3], color: [f32; 3], size: f32) -> Vec<Vertex> {
    let normal_front = [0.0, 0.0, 1.0];
    let min = -size / 2.0;
    let max = size / 2.0;
    // front
    let vertices = vec![
        Vertex::new([pos[0] + min, pos[1] + min, pos[2] + max], normal_front, color),
        Vertex::new([pos[0] + max, pos[1] + min, pos[2] + max], normal_front, color),
        Vertex::new([pos[0] + min, pos[1] + max, pos[2] + max], normal_front, color),
        Vertex::new([pos[0] + max, pos[1] + min, pos[2] + max], normal_front, color),
        Vertex::new([pos[0] + max, pos[1] + max, pos[2] + max], normal_front, color),
        Vertex::new([pos[0] + min, pos[1] + max, pos[2] + max], normal_front, color),
    ];
    vertices
}

fn top_face(pos: [f32; 3], color: [f32; 3], size: f32) -> Vec<Vertex> {
    let normal_top = [0.0, 1.0, 0.0];
    let min = -size / 2.0;
    let max = size / 2.0;
    let vertices = vec![
        Vertex::new([pos[0] + min, pos[1] + max, pos[2] + min], normal_top, color),
        Vertex::new([pos[0] + min, pos[1] + max, pos[2] + max], normal_top, color),
        Vertex::new([pos[0] + max, pos[1] + max, pos[2] + min], normal_top, color),
        Vertex::new([pos[0] + max, pos[1] + max, pos[2] + min], normal_top, color),
        Vertex::new([pos[0] + min, pos[1] + max, pos[2] + max], normal_top, color),
        Vertex::new([pos[0] + max, pos[1] + max, pos[2] + max], normal_top, color),
    ];
    vertices
}
fn bottom_face(pos: [f32; 3], color: [f32; 3], size: f32) -> Vec<Vertex> {
    let normal_bottom = [0.0, -1.0, 0.0];
    let min = -size / 2.0;
    let max = size / 2.0;
    let vertices = vec![
        Vertex::new([pos[0] + min, pos[1] + min, pos[2] + min], normal_bottom, color),
        Vertex::new([pos[0] + max, pos[1] + min, pos[2] + min], normal_bottom, color),
        Vertex::new([pos[0] + min, pos[1] + min, pos[2] + max], normal_bottom, color),
        Vertex::new([pos[0] + max, pos[1] + min, pos[2] + min], normal_bottom, color),
        Vertex::new([pos[0] + max, pos[1] + min, pos[2] + max], normal_bottom, color),
        Vertex::new([pos[0] + min, pos[1] + min, pos[2] + max], normal_bottom, color),
    ];
    vertices
}
fn right_face(pos: [f32; 3], color: [f32; 3], size: f32) -> Vec<Vertex> {
    let normal_right = [1.0, 0.0, 0.0];
    let min = -size / 2.0;
    let max = size / 2.0;
    let vertices = vec![
        Vertex::new([pos[0] + max, pos[1] + min, pos[2] + min], normal_right, color),
        Vertex::new([pos[0] + max, pos[1] + max, pos[2] + min], normal_right, color),
        Vertex::new([pos[0] + max, pos[1] + min, pos[2] + max], normal_right, color),
        Vertex::new([pos[0] + max, pos[1] + max, pos[2] + min], normal_right, color),
        Vertex::new([pos[0] + max, pos[1] + max, pos[2] + max], normal_right, color),
        Vertex::new([pos[0] + max, pos[1] + min, pos[2] + max], normal_right, color),
    ];
    vertices
}
fn left_face(pos: [f32; 3], color: [f32; 3], size: f32) -> Vec<Vertex> {
    let normal_left = [-1.0, 0.0, 0.0];
    let min = -size / 2.0;
    let max = size / 2.0;
    let vertices = vec![
        Vertex::new([pos[0] + min, pos[1] + min, pos[2] + max], normal_left, color),
        Vertex::new([pos[0] + min, pos[1] + max, pos[2] + max], normal_left, color),
        Vertex::new([pos[0] + min, pos[1] + min, pos[2] + min], normal_left, color),
        Vertex::new([pos[0] + min, pos[1] + max, pos[2] + max], normal_left, color),
        Vertex::new([pos[0] + min, pos[1] + max, pos[2] + min], normal_left, color),
        Vertex::new([pos[0] + min, pos[1] + min, pos[2] + min], normal_left, color),
    ];
    vertices
}
fn back_face(pos: [f32; 3], color: [f32; 3], size: f32) -> Vec<Vertex> {
    let normal_back = [0.0, 0.0, -1.0];
    let min = -size / 2.0;
    let max = size / 2.0;
    let vertices = vec![
        Vertex::new([pos[0] + min, pos[1] + max, pos[2] + min], normal_back, color),
        Vertex::new([pos[0] + max, pos[1] + max, pos[2] + min], normal_back, color),
        Vertex::new([pos[0] + min, pos[1] + min, pos[2] + min], normal_back, color),
        Vertex::new([pos[0] + max, pos[1] + max, pos[2] + min], normal_back, color),
        Vertex::new([pos[0] + max, pos[1] + min, pos[2] + min], normal_back, color),
        Vertex::new([pos[0] + min, pos[1] + min, pos[2] + min], normal_back, color),
    ];
    vertices
}

fn make_cube(vertices: &mut Vec<Vertex>, indices: &mut Vec<u32>, pos: [f32; 3], color: [f32; 3], size: f32) {
    let count = vertices.len() as u32;
    vertices.extend_from_slice(front_face(pos, color, size).as_slice());
    vertices.extend_from_slice(top_face(pos, color, size).as_slice());
    vertices.extend_from_slice(bottom_face(pos, color, size).as_slice());
    vertices.extend_from_slice(back_face(pos, color, size).as_slice());
    vertices.extend_from_slice(left_face(pos, color, size).as_slice());
    vertices.extend_from_slice(right_face(pos, color, size).as_slice());
    indices.extend((0..36).into_iter().map(|i| count + i));
}

pub fn load_vox(buffer: &[u8], mut add_mesh: impl FnMut(Mesh) -> Handle<Mesh>) {
    let data = dot_vox::load_bytes(buffer).unwrap();
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut count = 0;
    for model in data.models.iter() {
        for voxel in model.voxels.iter() {
            count += 1;
            let color = palette_to_color(data.palette[voxel.i as usize]);
            let size = 0.1;
            let pos = [voxel.x as f32 * size, voxel.z as f32 * size, voxel.y as f32 * size];
            make_cube(&mut vertices, &mut indices, pos, color, size);
        }
    }
    println!(
        "cubes: {}, triangles: {}, vertices: {}",
        count,
        count * 12,
        count * 12 * 3
    );
    add_mesh(Mesh {
        vertices,
        indices,
        just_loaded: true,
    });
}

pub struct Vox {
    data: Vec<Option<u8>>,
    palette: HashMap<u8, [f32; 3]>,
    pub x_size: usize,
    pub y_size: usize,
    pub z_size: usize,
}

impl Vox {
    pub fn new(x_size: usize, y_size: usize, z_size: usize) -> Self {
        Self {
            data: vec![None; z_size * y_size * x_size],
            palette: HashMap::default(),
            x_size,
            y_size,
            z_size,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, color_id: u8, color: [f32; 3]) {
        self.data[z * self.y_size * self.x_size + y * self.x_size + x] = Some(color_id);
        self.palette.insert(color_id, color);
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        self.data[z * self.y_size * self.x_size + y * self.x_size + x]
    }

    pub fn get_color(&self, color_id: u8) -> [f32; 3] {
        self.palette[&color_id]
    }
}

pub struct VoxModel {
    data: Vec<Option<u8>>,
    size: usize,
}

impl VoxModel {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![None; size * size * size],
            size,
        }
    }

    pub fn set(&mut self, x: i32, y: i32, z: i32, color_id: u8) {
        assert!(x >= 0 && x < self.size as i32);
        assert!(y >= 0 && y < self.size as i32);
        assert!(z >= 0 && z < self.size as i32);
        self.data[z as usize * self.size * self.size + y as usize * self.size + x as usize] = Some(color_id);
    }

    pub fn get(&self, x: i32, y: i32, z: i32) -> Option<u8> {
        if x >= 0 && y >= 0 && z >= 0 && x < self.size as i32 && y < self.size as i32 && z < self.size as i32 {
            self.data[z as usize * self.size * self.size + y as usize * self.size + x as usize]
        } else {
            None
        }
    }
}
struct Mask {
    data: Vec<Option<u8>>,
    size: usize,
}

impl Mask {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![None; size * size],
            size,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, color_id: Option<u8>) {
        assert!(x < self.size);
        assert!(y < self.size);
        self.data[y * self.size + x] = color_id;
    }

    pub fn get(&mut self, x: usize, y: usize) -> Option<u8> {
        self.data[y * self.size + x]
    }
}

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

pub fn load_world(mut add_mesh: impl FnMut(Mesh) -> Handle<Mesh>) {
    let buffer = std::fs::read("res/vox-models/#treehouse/#treehouse.vox").unwrap();
    if let Ok(data) = dot_vox::load_bytes(buffer.as_slice()) {
        if data.models.len() > 0 {
            println!(
                "{} {} {}",
                data.models[0].size.x, data.models[0].size.z, data.models[0].size.y
            );
            for voxel in data.models[0].voxels.iter() {}
        }
    }
}

pub fn load_dotvox_model(data: &dot_vox::DotVoxData, registry: &mut Registry<Vox>) -> Handle<Vox> {
    let model = &data.models[0];
    let mut vox_model = Vox::new(model.size.x as usize, model.size.z as usize, model.size.y as usize);
    for v in &model.voxels {
        let color = palette_to_color(data.palette[v.i as usize]);
        vox_model.set(v.x as usize, v.z as usize, v.y as usize, v.i, color);
    }
    registry.add(vox_model)
}

pub fn load_voxel_grid_as_mesh(mut add_mesh: impl FnMut(Mesh) -> Handle<Mesh>) {
    let chunk_size = 128;
    for file in test_files() {
        let mut voxel_grid = VoxModel::new(chunk_size);
        let buffer = std::fs::read(file).unwrap();
        if let Ok(data) = dot_vox::load_bytes(buffer.as_slice()) {
            // only load the first model
            if data.models.len() > 0 {
                println!(
                    "{} {} {}",
                    data.models[0].size.x, data.models[0].size.z, data.models[0].size.y
                );
                for voxel in data.models[0].voxels.iter() {
                    voxel_grid.set(voxel.x as i32, voxel.z as i32, voxel.y as i32, voxel.i);
                }
                greedy_mesh(voxel_grid, data.palette.as_slice(), &mut add_mesh);
            }
        }
    }
}

fn greedy_mesh(voxel_grid: VoxModel, color_table: &[u32], mut add_mesh: impl FnMut(Mesh) -> Handle<Mesh>) {
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

    for d in descriptors.iter() {
        let u = d.u;
        let v = d.v;
        let w = d.w;
        let normal = d.normal;
        let normal_outside = [-(normal[0] as f32), -(normal[1] as f32), -(normal[2] as f32)];

        for slice in 0..voxel_grid.size {
            let slice = if d.step == 1 {
                slice
            } else {
                voxel_grid.size - (slice + 1)
            };
            let mut cursor = [0, 0, 0];
            cursor[u] = slice;
            let mut mask = Mask::new(voxel_grid.size as usize);
            for cursor_w in 0..voxel_grid.size {
                for cursor_v in 0..voxel_grid.size {
                    cursor[v] = cursor_v;
                    cursor[w] = cursor_w;
                    let voxel_back = voxel_grid.get(
                        cursor[0] as i32 - normal[0] as i32,
                        cursor[1] as i32 - normal[1] as i32,
                        cursor[2] as i32 - normal[2] as i32,
                    );
                    let voxel = voxel_grid.get(cursor[0] as i32, cursor[1] as i32, cursor[2] as i32);
                    let color_id = if voxel_back != None && voxel != None && voxel_back == voxel {
                        None
                    } else {
                        voxel
                    };
                    mask.set(cursor[v], cursor[w], color_id);
                }
            }
            for y in 0..voxel_grid.size {
                for mut x in 0..voxel_grid.size {
                    let color_id = mask.get(x, y);
                    if let Some(m) = color_id {
                        let mut width = 1;
                        while x + width < voxel_grid.size && mask.get(x + width, y) == color_id {
                            width += 1;
                        }
                        let mut height = 1;
                        let mut done = false;
                        while y + height < voxel_grid.size && !done {
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

                        let color = palette_to_color(color_table[m as usize]);
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
                        x += width;
                    }
                }
            }
        }
    }
    add_mesh(Mesh {
        vertices,
        indices,
        just_loaded: true,
    });
}

fn palette_to_color(from: u32) -> [f32; 3] {
    let (_a, b, g, r) = (from >> 24 & 0xFF, from >> 16 & 0xFF, from >> 8 & 0xFF, from & 0xFF);
    [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
}

fn test_files() -> &'static [&'static str] {
    &[
        /*"res/vox-models/#skyscraper/#skyscraper_01_000.vox",
        "res/vox-models/#skyscraper/#skyscraper_02_000.vox",
        "res/vox-models/#skyscraper/#skyscraper_03_000.vox",
        "res/vox-models/#skyscraper/#skyscraper_06_000.vox",
        "res/vox-models/#skyscraper/#skyscraper_05_000.vox",
        "res/vox-models/#skyscraper/#skyscraper_04_000.vox",
        "res/vox-models/#haunted_house/#haunted_house.vox",
         */
        "res/vox-models/#treehouse/#treehouse.vox",
        /*
        "res/vox-models/#phantom_mansion/#phantom_mansion.vox",
        "res/vox-models/castle.vox",
         */
    ]
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {}
}
