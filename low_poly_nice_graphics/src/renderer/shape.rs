use crate::renderer::Vertex;

pub struct Shape {
    pub vertices: Vec<Vertex>,
}

pub struct Plane {
    size: f32,
    subdivisions: u32,
}

impl Plane {
    pub fn new(size: f32, subdivisions: u32) -> Self {
        Self { size, subdivisions }
    }
}

impl From<Plane> for Shape {
    fn from(terrain: Plane) -> Self {
        let increments = 2i32.pow(terrain.subdivisions);
        let increment = terrain.size / increments as f32;
        let mut vertices = Vec::new();
        for x in 0..increments {
            for z in 0..increments {
                let x = x - increments / 2;
                let z = z - increments / 2;
                vertices.extend_from_slice(&[
                    Vertex {
                        position: [x as f32 * increment, 0.0, z as f32 * increment],
                        normal: [0.0, 1.0, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: [x as f32 * increment, 0.0, (z + 1) as f32 * increment],
                        normal: [0.0, 1.0, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: [(x + 1) as f32 * increment, 0.0, (z + 1) as f32 * increment],
                        normal: [0.0, 1.0, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: [x as f32 * increment, 0.0, z as f32 * increment],
                        normal: [0.0, 1.0, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: [(x + 1) as f32 * increment, 0.0, (z + 1) as f32 * increment],
                        normal: [0.0, 1.0, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: [(x + 1) as f32 * increment, 0.0, z as f32 * increment],
                        normal: [0.0, 1.0, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                ]);
            }
        }
        Self { vertices }
    }
}
