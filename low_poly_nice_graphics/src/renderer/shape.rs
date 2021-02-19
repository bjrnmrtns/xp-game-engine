use crate::{
    generators::{Height, Zero},
    renderer::Vertex,
};

pub struct Shape {
    pub vertices: Vec<Vertex>,
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
                        color: [0.86, 0.86, 0.86],
                    },
                    Vertex {
                        position: p01,
                        normal: n0,
                        color: [0.86, 0.86, 0.86],
                    },
                    Vertex {
                        position: p11,
                        normal: n0,
                        color: [0.86, 0.86, 0.86],
                    },
                    Vertex {
                        position: p00,
                        normal: n1,
                        color: [0.86, 0.86, 0.86],
                    },
                    Vertex {
                        position: p11,
                        normal: n1,
                        color: [0.86, 0.86, 0.86],
                    },
                    Vertex {
                        position: p10,
                        normal: n1,
                        color: [0.86, 0.86, 0.86],
                    },
                ]);
            }
        }
        Self { vertices }
    }
}

pub struct Cube {
    size: f32,
}

impl Cube {
    pub fn new(size: f32) -> Self {
        Self { size }
    }
}

impl From<Cube> for Shape {
    fn from(cube: Cube) -> Self {
        let max = cube.size / 2.0;
        let min = -max;
        let color = [1.0, 1.0, 1.0];
        let mut vertices = Vec::new();
        let normal_top = [0.0, 1.0, 0.0];
        let normal_bottom = [0.0, -1.0, 0.0];
        let normal_right = [1.0, 0.0, 0.0];
        let normal_left = [-1.0, 0.0, 0.0];
        let normal_front = [0.0, 0.0, 1.0];
        let normal_back = [0.0, 0.0, -1.0];
        vertices.extend_from_slice(&[
            // top
            Vertex::new([min, max, min], normal_top, color),
            Vertex::new([min, max, max], normal_top, color),
            Vertex::new([max, max, min], normal_top, color),
            Vertex::new([max, max, min], normal_top, color),
            Vertex::new([min, max, max], normal_top, color),
            Vertex::new([max, max, max], normal_top, color),
            // bottom
            Vertex::new([min, min, min], normal_bottom, color),
            Vertex::new([max, min, min], normal_bottom, color),
            Vertex::new([min, min, max], normal_bottom, color),
            Vertex::new([max, min, min], normal_bottom, color),
            Vertex::new([max, min, max], normal_bottom, color),
            Vertex::new([min, min, max], normal_bottom, color),
            // right
            Vertex::new([max, min, min], normal_right, color),
            Vertex::new([max, max, min], normal_right, color),
            Vertex::new([max, min, max], normal_right, color),
            Vertex::new([max, max, min], normal_right, color),
            Vertex::new([max, max, max], normal_right, color),
            Vertex::new([max, min, max], normal_right, color),
            // left
            Vertex::new([min, min, max], normal_left, color),
            Vertex::new([min, max, max], normal_left, color),
            Vertex::new([min, min, min], normal_left, color),
            Vertex::new([min, max, max], normal_left, color),
            Vertex::new([min, max, min], normal_left, color),
            Vertex::new([min, min, min], normal_left, color),
            // front
            Vertex::new([min, min, max], normal_front, color),
            Vertex::new([max, min, max], normal_front, color),
            Vertex::new([min, max, max], normal_front, color),
            Vertex::new([max, min, max], normal_front, color),
            Vertex::new([max, max, max], normal_front, color),
            Vertex::new([min, max, max], normal_front, color),
            // back
            Vertex::new([min, max, min], normal_back, color),
            Vertex::new([max, max, min], normal_back, color),
            Vertex::new([min, min, min], normal_back, color),
            Vertex::new([max, max, min], normal_back, color),
            Vertex::new([max, min, min], normal_back, color),
            Vertex::new([min, min, min], normal_back, color),
        ]);
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
