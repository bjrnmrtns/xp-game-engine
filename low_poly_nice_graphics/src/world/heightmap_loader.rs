use crate::{
    mesh::{triangle_normal, Mesh, Vertex},
    registry::Handle,
};
use image::GenericImageView;

pub struct Heightmap;

impl Heightmap {
    pub fn load_heightmap(mut add_mesh: impl FnMut(Mesh) -> Handle<Mesh>) {
        let world_image = image::load_from_memory(std::fs::read("res/map/heightmap.jpg").unwrap().as_slice()).unwrap();
        let offset_x = 2800i32;
        let offset_z = 2000i32;
        let width = 200;
        let height = 200;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let height_divisor = 10.0;
        for x in 0..height {
            for z in 0..width {
                let remap_x = x - width / 2;
                let remap_z = z - width / 2;
                let p00 = [
                    remap_x as f32,
                    world_image.get_pixel((x + offset_x) as u32, (z + offset_z) as u32).0[0] as f32 / height_divisor,
                    remap_z as f32,
                ];
                let p10 = [
                    remap_x as f32 + 1.0,
                    world_image
                        .get_pixel((x + offset_x + 1) as u32, (z + offset_z) as u32)
                        .0[0] as f32
                        / height_divisor,
                    remap_z as f32,
                ];
                let p01 = [
                    remap_x as f32,
                    world_image
                        .get_pixel((x + offset_x) as u32, (z + offset_z + 1) as u32)
                        .0[0] as f32
                        / height_divisor,
                    remap_z as f32 + 1.0,
                ];
                let p11 = [
                    remap_x as f32 + 1.0,
                    world_image
                        .get_pixel((x + offset_x + 1) as u32, (z + offset_z + 1) as u32)
                        .0[0] as f32
                        / height_divisor,
                    remap_z as f32 + 1.0,
                ];
                let color = [0.0, 0.0, 1.0];
                let normal_first: [f32; 3] = triangle_normal(p00, p01, p11);
                let normal_second: [f32; 3] = triangle_normal(p00, p11, p10);
                let count = vertices.len() as u32;
                vertices.extend_from_slice(&[
                    Vertex::new(p00, normal_first, color),
                    Vertex::new(p01, normal_first, color),
                    Vertex::new(p11, normal_first, color),
                    Vertex::new(p00, normal_second, color),
                    Vertex::new(p11, normal_second, color),
                    Vertex::new(p10, normal_second, color),
                ]);
                indices.extend((0..6).into_iter().map(|i| count + i));
            }
        }
        add_mesh(Mesh {
            vertices,
            indices,
            just_loaded: true,
        });
    }
}
