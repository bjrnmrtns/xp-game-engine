use crate::{
    mesh::{Cube, Mesh, Vertex},
    registry::Handle,
};

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

pub fn load_vox(buffer: &[u8], mut add_mesh: impl FnMut(Mesh) -> Handle<Mesh>) {
    let data = dot_vox::load_bytes(buffer).unwrap();
    let mut vertices = Vec::new();
    let mut count = 0;
    for model in data.models.iter() {
        for voxel in model.voxels.iter() {
            count += 1;
            let color = palette_to_color(data.palette[voxel.i as usize]);
            let size = 0.1;
            let pos = [voxel.x as f32 * size, voxel.z as f32 * size, voxel.y as f32 * size];
            vertices.extend_from_slice(front_face(pos, color, size).as_slice());
            vertices.extend_from_slice(top_face(pos, color, size).as_slice());
            vertices.extend_from_slice(bottom_face(pos, color, size).as_slice());
            vertices.extend_from_slice(back_face(pos, color, size).as_slice());
            vertices.extend_from_slice(left_face(pos, color, size).as_slice());
            vertices.extend_from_slice(right_face(pos, color, size).as_slice());
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
        just_loaded: true,
    });
}

struct VoxelGrid {
    data: Vec<Option<u8>>,
    size: usize,
}

impl VoxelGrid {
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

    pub fn get(&mut self, x: i32, y: i32, z: i32) -> Option<u8> {
        if x >= 0 && y >= 0 && z >= 0 {
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

pub fn load_test_vox_files(mut add_mesh: impl FnMut(Mesh) -> Handle<Mesh>) {
    let mut count = 0;
    let mut vertices = Vec::new();
    let mut offset = 0;
    for file in test_files() {
        let buffer = std::fs::read(file).unwrap();
        if let Ok(data) = dot_vox::load_bytes(buffer.as_slice()) {
            for model in data.models.iter() {
                for voxel in model.voxels.iter() {
                    count += 1;
                    let color = palette_to_color(data.palette[voxel.i as usize]);
                    let size = 0.1;
                    let pos = [voxel.x as u32, voxel.z as u32, voxel.y as u32 + offset];
                    let pos = [pos[0] as f32 * size, pos[1] as f32 * size, pos[2] as f32 * size];
                    vertices.extend_from_slice(front_face(pos, color, size).as_slice());
                    vertices.extend_from_slice(top_face(pos, color, size).as_slice());
                    vertices.extend_from_slice(bottom_face(pos, color, size).as_slice());
                    vertices.extend_from_slice(back_face(pos, color, size).as_slice());
                    vertices.extend_from_slice(left_face(pos, color, size).as_slice());
                    vertices.extend_from_slice(right_face(pos, color, size).as_slice());
                }
                offset += 128;
            }
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
        just_loaded: true,
    });
}

pub fn load_test_vox_files_culling(mut add_mesh: impl FnMut(Mesh) -> Handle<Mesh>) {
    let mut vertices = Vec::new();
    let chunk_size = 8;
    let mut voxel_grid = VoxelGrid::new(chunk_size);
    voxel_grid.set(0, 0, 0, 1);
    voxel_grid.set(1, 0, 0, 1);
    voxel_grid.set(2, 0, 0, 1);
    voxel_grid.set(0, 2, 0, 1);
    voxel_grid.set(1, 2, 0, 1);
    voxel_grid.set(0, 3, 0, 1);
    for norm in 0..3 {
        let tan = (norm + 1) % 3;
        let bi_tan = (norm + 2) % 3;
        let mut normal = [0, 0, 0];
        normal[norm] = 1;
        let normal = normal;
        let normal_outside = [-(normal[0] as f32), -(normal[1] as f32), -(normal[2] as f32)];

        for slice in 0..chunk_size {
            let mut cursor = [0, 0, 0];
            cursor[norm] = slice;
            let mut mask = Mask::new(chunk_size as usize);
            for cursor_bi_tan in 0..chunk_size {
                for cursor_tan in 0..chunk_size {
                    cursor[tan] = cursor_tan;
                    cursor[bi_tan] = cursor_bi_tan;
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
                    mask.set(cursor[tan], cursor[bi_tan], color_id);
                }
            }
            for y in 0..chunk_size {
                for mut x in 0..chunk_size {
                    let color_id = mask.get(x, y);
                    if let Some(m) = color_id {
                        let mut width = 1;
                        while x + width < chunk_size && mask.get(x + width, y) == color_id {
                            width += 1;
                        }
                        let mut height = 1;
                        let mut done = false;
                        while y + height < chunk_size && !done {
                            let mut k = x;
                            while k < width && !done {
                                if mask.get(k, y + height) == color_id {
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
                        base[norm] = slice as f32;
                        base[tan] = x as f32;
                        base[bi_tan] = y as f32;

                        let mut du = [0.0, 0.0, 0.0];
                        du[tan] = width as f32;
                        let mut dv = [0.0, 0.0, 0.0];
                        dv[bi_tan] = height as f32;

                        let color = [1.0, 0.0, 0.0];
                        vertices.extend_from_slice(&[
                            Vertex::new([base[0], base[1], base[2]], normal_outside, color),
                            Vertex::new(
                                [
                                    base[0] + du[0] + dv[0],
                                    base[1] + du[1] + dv[1],
                                    base[2] + du[2] + dv[2],
                                ],
                                normal_outside,
                                color,
                            ),
                            Vertex::new(
                                [base[0] + du[0], base[1] + du[1], base[2] + du[2]],
                                normal_outside,
                                color,
                            ),
                            Vertex::new([base[0], base[1], base[2]], normal_outside, color),
                            Vertex::new(
                                [base[0] + dv[0], base[1] + dv[1], base[2] + dv[2]],
                                normal_outside,
                                color,
                            ),
                            Vertex::new(
                                [
                                    base[0] + du[0] + dv[0],
                                    base[1] + du[1] + dv[1],
                                    base[2] + du[2] + dv[2],
                                ],
                                normal_outside,
                                color,
                            ),
                        ]);
                        for yy in y..height {
                            for xx in x..width {
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
         */
        //"res/vox-models/#haunted_house/#haunted_house.vox",
        //        "res/vox-models/#treehouse/#treehouse.vox",
        "res/vox-models/#phantom_mansion/#phantom_mansion.vox",
        //"res/vox-models/castle.vox",
    ]
}

#[cfg(test)]
mod tests {
    use crate::{
        registry::Registry,
        voxel::vox_loader::{load_test_vox_files_culling, load_vox},
    };

    #[test]
    fn test() {
        load_test_vox_files_culling(|mesh| {
            let mut registry = Registry::new();
            let mesh = registry.add(mesh);
            mesh
        });
    }
}
