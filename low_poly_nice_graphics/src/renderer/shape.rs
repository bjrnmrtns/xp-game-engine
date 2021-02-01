use crate::renderer::Vertex;

pub struct Shape {
    pub vertices: Vec<Vertex>,
}

pub trait Height {
    fn height(&self, x: f32, y: f32) -> f32;
}

struct Zero;

impl Height for Zero {
    fn height(&self, _x: f32, _y: f32) -> f32 {
        0.0
    }
}

pub struct Plane {
    size: f32,
    subdivisions: u32,
    height_function: Box<dyn Height>,
}

impl Plane {
    pub fn new(size: f32) -> Self {
        Self {
            size,
            subdivisions: 0,
            height_function: Box::new(Zero),
        }
    }
}

impl From<Plane> for Shape {
    fn from(plane: Plane) -> Self {
        let increments = 2i32.pow(plane.subdivisions);
        let increment = plane.size / increments as f32;
        let mut vertices = Vec::new();
        for x in 0..increments {
            for z in 0..increments {
                let x = x as f32 - increments as f32 / 2.0;
                let z = z as f32 - increments as f32 / 2.0;
                vertices.extend_from_slice(&[
                    Vertex {
                        position: [
                            x * increment,
                            plane.height_function.height(x * increment, z * increment),
                            z * increment,
                        ],
                        normal: [0.0, 1.0, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: [
                            x * increment,
                            plane
                                .height_function
                                .height(x * increment, (z + 1.0) * increment),
                            (z + 1.0) * increment,
                        ],
                        normal: [0.0, 1.0, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: [
                            (x + 1.0) * increment,
                            plane
                                .height_function
                                .height((x + 1.0) * increment, (z + 1.0) * increment),
                            (z + 1.0) * increment,
                        ],
                        normal: [0.0, 1.0, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: [
                            x * increment,
                            plane.height_function.height(x * increment, z * increment),
                            z * increment,
                        ],
                        normal: [0.0, 1.0, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: [
                            (x + 1.0) * increment,
                            plane
                                .height_function
                                .height((x + 1.0) * increment, (z + 1.0) * increment),
                            (z + 1.0) * increment,
                        ],
                        normal: [0.0, 1.0, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: [
                            (x + 1.0) * increment,
                            plane
                                .height_function
                                .height((x + 1.0) * increment, z * increment),
                            z * increment,
                        ],
                        normal: [0.0, 1.0, 0.0],
                        color: [0.0, 0.0, 1.0],
                    },
                ]);
            }
        }
        Self { vertices }
    }
}
