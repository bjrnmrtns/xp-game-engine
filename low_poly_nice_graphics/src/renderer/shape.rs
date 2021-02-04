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
    pub fn new(size: f32, subdivisions: u32, height_function: Box<dyn Height>) -> Self {
        Self {
            size,
            subdivisions,
            height_function,
        }
    }

    pub fn flat(size: f32) -> Self {
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
                let p00 = [
                    x * increment,
                    plane.height_function.height(x * increment, z * increment),
                    z * increment,
                ];
                let p01 = [
                    x * increment,
                    plane
                        .height_function
                        .height(x * increment, (z + 1.0) * increment),
                    (z + 1.0) * increment,
                ];
                let p10 = [
                    (x + 1.0) * increment,
                    plane
                        .height_function
                        .height((x + 1.0) * increment, z * increment),
                    z * increment,
                ];
                let p11 = [
                    (x + 1.0) * increment,
                    plane
                        .height_function
                        .height((x + 1.0) * increment, (z + 1.0) * increment),
                    (z + 1.0) * increment,
                ];
                let n0 =
                    nalgebra_glm::triangle_normal(&p00.into(), &p01.into(), &p11.into()).into();
                let n1 =
                    nalgebra_glm::triangle_normal(&p00.into(), &p11.into(), &p10.into()).into();
                vertices.extend_from_slice(&[
                    Vertex {
                        position: p00,
                        normal: n0,
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: p01,
                        normal: n0,
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: p11,
                        normal: n0,
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: p00,
                        normal: n1,
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: p11,
                        normal: n1,
                        color: [0.0, 0.0, 1.0],
                    },
                    Vertex {
                        position: p10,
                        normal: n1,
                        color: [0.0, 0.0, 1.0],
                    },
                ]);
            }
        }
        Self { vertices }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_understanding_normal_calculation_0() {
        // counter clockwise triangle
        let p00 = [0.0, 0.0, 0.0];
        let p01 = [0.0, 0.0, 1.0];
        let p10 = [1.0, 0.0, 0.0];
        let normalized_normal: [f32; 3] =
            nalgebra_glm::triangle_normal(&p00.into(), &p01.into(), &p10.into()).into();
        assert_eq!([0.0, 1.0, 0.0], normalized_normal);
    }
    #[test]
    fn check_understanding_normal_calculation_1() {
        // counter clockwise triangle
        let p00 = [-1.0, 0.0, -1.0];
        let p01 = [0.0, 0.0, 0.0];
        let p10 = [0.0, 0.0, -1.0];
        let normalized_normal: [f32; 3] =
            nalgebra_glm::triangle_normal(&p00.into(), &p01.into(), &p10.into()).into();
        assert_eq!([0.0, 1.0, 0.0], normalized_normal);
    }
}
