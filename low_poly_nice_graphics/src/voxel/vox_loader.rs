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
    ]
}

#[cfg(test)]
mod tests {
    use crate::{registry::Registry, voxel::vox_loader::load_vox};

    #[test]
    fn test() {
        let file_data = std::fs::read("example.vox").unwrap();
        load_vox(file_data.as_slice(), |mesh| {
            let mut registry = Registry::new();
            let mesh = registry.add(mesh);
            mesh
        });
    }
}
