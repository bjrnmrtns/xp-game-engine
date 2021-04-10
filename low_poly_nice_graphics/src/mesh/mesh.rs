use crate::{
    generators::{Height, Zero},
    mesh::Vertex,
    tiles::{Tile, TileConfiguration, TileType},
};
use glam::Vec3;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub just_loaded: bool,
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

pub fn triangle_normal(p0: [f32; 3], p1: [f32; 3], p2: [f32; 3]) -> [f32; 3] {
    let edge0 = Vec3::from(p2) - Vec3::from(p0);
    let edge1 = Vec3::from(p1) - Vec3::from(p0);
    edge1.cross(edge0).into()
}

impl From<Plane> for Mesh {
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
                    plane.height_function.height(x * increment, (z + 1.0) * increment),
                    (z + 1.0) * increment,
                ];
                let p10 = [
                    (x + 1.0) * increment,
                    plane.height_function.height((x + 1.0) * increment, z * increment),
                    z * increment,
                ];
                let p11 = [
                    (x + 1.0) * increment,
                    plane
                        .height_function
                        .height((x + 1.0) * increment, (z + 1.0) * increment),
                    (z + 1.0) * increment,
                ];

                let n0 = triangle_normal(p00, p01, p11);
                let n1 = triangle_normal(p00, p11, p10);
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
        Self {
            vertices,
            just_loaded: true,
        }
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

impl From<Cube> for Mesh {
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
        Self {
            vertices,
            just_loaded: true,
        }
    }
}

pub struct IcoSphere {
    radius: f32,
    subdivisions: usize,
}

impl IcoSphere {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            subdivisions: 1,
        }
    }
}

fn add_midpoint(
    vertices: &mut Vec<[f32; 3]>,
    lookup: &mut HashMap<(usize, usize), usize>,
    v0: usize,
    v1: usize,
) -> usize {
    let edge = if v0 < v1 { (v0, v1) } else { (v1, v0) };
    if !lookup.contains_key(&edge) {
        lookup.insert(edge, vertices.len());
        // normalize works because we are working with unit vectors
        let midpoint = (Vec3::from(vertices[v0]) + Vec3::from(vertices[v1])).normalize();
        vertices.push(midpoint.into());
    }
    *lookup.get(&edge).unwrap()
}

fn sphere_subdivide(vertices: &mut Vec<[f32; 3]>, triangles: &[[usize; 3]]) -> Vec<[usize; 3]> {
    let mut lookup = HashMap::new();
    let mut result = Vec::new();
    for triangle in triangles {
        let mid0 = add_midpoint(vertices, &mut lookup, triangle[0], triangle[1]);
        let mid1 = add_midpoint(vertices, &mut lookup, triangle[1], triangle[2]);
        let mid2 = add_midpoint(vertices, &mut lookup, triangle[2], triangle[0]);

        result.push([triangle[0], mid0, mid2]);
        result.push([triangle[1], mid1, mid0]);
        result.push([triangle[2], mid2, mid1]);
        result.push([mid0, mid1, mid2]);
    }
    result
}

fn scale_vertex(v: [f32; 3], scale: f32) -> [f32; 3] {
    [v[0] * scale, v[1] * scale, v[2] * scale]
}

impl From<IcoSphere> for Mesh {
    fn from(sphere: IcoSphere) -> Self {
        const X: f32 = 0.525731112119133606;
        const Z: f32 = 0.850650808352039932;
        const N: f32 = 0.0;

        let color = [1.0, 0.0, 0.0];

        let mut points = vec![
            [-X, N, Z],
            [X, N, Z],
            [-X, N, -Z],
            [X, N, -Z],
            [N, Z, X],
            [N, Z, -X],
            [N, -Z, X],
            [N, -Z, -X],
            [Z, X, N],
            [-Z, X, N],
            [Z, -X, N],
            [-Z, -X, N],
        ];
        let mut triangles = vec![
            [0, 1, 4],
            [0, 4, 9],
            [9, 4, 5],
            [4, 8, 5],
            [4, 1, 8],
            [8, 1, 10],
            [8, 10, 3],
            [5, 8, 3],
            [5, 3, 2],
            [2, 3, 7],
            [7, 3, 10],
            [7, 10, 6],
            [7, 6, 11],
            [11, 6, 0],
            [0, 6, 1],
            [6, 10, 1],
            [9, 11, 0],
            [9, 2, 11],
            [9, 5, 2],
            [7, 11, 2],
        ];
        for _ in 0..sphere.subdivisions {
            triangles = sphere_subdivide(&mut points, triangles.as_slice());
        }
        let mut vertices = Vec::new();
        for triangle in &triangles {
            let normal = triangle_normal(points[triangle[0]], points[triangle[1]], points[triangle[2]]);
            vertices.push(Vertex::new(
                scale_vertex(points[triangle[0]], sphere.radius),
                normal,
                color,
            ));
            vertices.push(Vertex::new(
                scale_vertex(points[triangle[1]], sphere.radius),
                normal,
                color,
            ));
            vertices.push(Vertex::new(
                scale_vertex(points[triangle[2]], sphere.radius),
                normal,
                color,
            ));
        }
        Self {
            vertices,
            just_loaded: true,
        }
    }
}

impl From<Tile> for Mesh {
    fn from(tile: Tile) -> Self {
        let mut vertices = Vec::new();
        let max = 0.5;
        let min = -0.5;
        let green = [0.0, 1.0, 0.0];
        let grey = [0.5, 0.5, 0.5];
        let pink = [1.0, 0.0, 1.0];
        let normal_left = [-1.0, 0.0, 0.0];
        let normal_right = [1.0, 0.0, 0.0];
        let normal_front = [0.0, 0.0, 1.0];
        let normal_back = [0.0, 0.0, -1.0];
        let normal_top = [0.0, 1.0, 0.0];
        match tile.tile_type {
            TileType::Empty => {
                vertices.extend_from_slice(&[
                    // ground plane
                    Vertex::new([min, 0.0, min], normal_top, pink),
                    Vertex::new([min, 0.0, max], normal_top, pink),
                    Vertex::new([max, 0.0, max], normal_top, pink),
                    Vertex::new([min, 0.0, min], normal_top, pink),
                    Vertex::new([max, 0.0, max], normal_top, pink),
                    Vertex::new([max, 0.0, min], normal_top, pink),
                ]);
            }
            TileType::Grass => match tile.configuration {
                _ => {
                    vertices.extend_from_slice(&[
                        // ground plane
                        Vertex::new([min, 0.0, min], normal_top, green),
                        Vertex::new([min, 0.0, max], normal_top, green),
                        Vertex::new([max, 0.0, max], normal_top, green),
                        Vertex::new([min, 0.0, min], normal_top, green),
                        Vertex::new([max, 0.0, max], normal_top, green),
                        Vertex::new([max, 0.0, min], normal_top, green),
                    ]);
                }
            },
            TileType::Stone => match tile.configuration {
                TileConfiguration::NoSides => {
                    vertices.extend_from_slice(&[
                        // ground plane
                        Vertex::new([min, 0.1, min], normal_top, grey),
                        Vertex::new([min, 0.1, max], normal_top, grey),
                        Vertex::new([max, 0.1, max], normal_top, grey),
                        Vertex::new([min, 0.1, min], normal_top, grey),
                        Vertex::new([max, 0.1, max], normal_top, grey),
                        Vertex::new([max, 0.1, min], normal_top, grey),
                    ]);
                }
                TileConfiguration::OneSide => {
                    vertices.extend_from_slice(&[
                        // ground plane
                        Vertex::new([min, 0.1, min], normal_top, grey),
                        Vertex::new([min, 0.1, max], normal_top, grey),
                        Vertex::new([max, 0.1, max], normal_top, grey),
                        Vertex::new([min, 0.1, min], normal_top, grey),
                        Vertex::new([max, 0.1, max], normal_top, grey),
                        Vertex::new([max, 0.1, min], normal_top, grey),
                        // back
                        Vertex::new([min, 0.0, min], normal_back, grey),
                        Vertex::new([min, 0.1, min], normal_back, grey),
                        Vertex::new([max, 0.0, min], normal_back, grey),
                        Vertex::new([max, 0.0, min], normal_back, grey),
                        Vertex::new([min, 0.1, min], normal_back, grey),
                        Vertex::new([max, 0.1, min], normal_back, grey),
                    ]);
                }

                TileConfiguration::BothSides => {
                    vertices.extend_from_slice(&[
                        // ground plane
                        Vertex::new([min, 0.1, min], normal_top, grey),
                        Vertex::new([min, 0.1, max], normal_top, grey),
                        Vertex::new([max, 0.1, max], normal_top, grey),
                        Vertex::new([min, 0.1, min], normal_top, grey),
                        Vertex::new([max, 0.1, max], normal_top, grey),
                        Vertex::new([max, 0.1, min], normal_top, grey),
                        // left
                        Vertex::new([min, 0.0, min], normal_left, grey),
                        Vertex::new([min, 0.0, max], normal_left, grey),
                        Vertex::new([min, 0.1, min], normal_left, grey),
                        Vertex::new([min, 0.1, min], normal_left, grey),
                        Vertex::new([min, 0.0, max], normal_left, grey),
                        Vertex::new([min, 0.1, max], normal_left, grey),
                        // right
                        Vertex::new([max, 0.0, min], normal_right, grey),
                        Vertex::new([max, 0.1, min], normal_right, grey),
                        Vertex::new([max, 0.0, max], normal_right, grey),
                        Vertex::new([max, 0.1, min], normal_right, grey),
                        Vertex::new([max, 0.1, max], normal_right, grey),
                        Vertex::new([max, 0.0, max], normal_right, grey),
                    ]);
                }
                TileConfiguration::Corner => {
                    vertices.extend_from_slice(&[
                        // ground plane
                        Vertex::new([min, 0.1, min], normal_top, grey),
                        Vertex::new([min, 0.1, max], normal_top, grey),
                        Vertex::new([max, 0.1, max], normal_top, grey),
                        Vertex::new([min, 0.1, min], normal_top, grey),
                        Vertex::new([max, 0.1, max], normal_top, grey),
                        Vertex::new([max, 0.1, min], normal_top, grey),
                        // back
                        Vertex::new([min, 0.0, min], normal_back, grey),
                        Vertex::new([min, 0.1, min], normal_back, grey),
                        Vertex::new([max, 0.0, min], normal_back, grey),
                        Vertex::new([max, 0.0, min], normal_back, grey),
                        Vertex::new([min, 0.1, min], normal_back, grey),
                        Vertex::new([max, 0.1, min], normal_back, grey),
                        // left
                        Vertex::new([min, 0.0, min], normal_left, grey),
                        Vertex::new([min, 0.0, max], normal_left, grey),
                        Vertex::new([min, 0.1, min], normal_left, grey),
                        Vertex::new([min, 0.1, min], normal_left, grey),
                        Vertex::new([min, 0.0, max], normal_left, grey),
                        Vertex::new([min, 0.1, max], normal_left, grey),
                    ]);
                }
                TileConfiguration::USide => {
                    vertices.extend_from_slice(&[
                        // ground plane
                        Vertex::new([min, 0.1, min], normal_top, grey),
                        Vertex::new([min, 0.1, max], normal_top, grey),
                        Vertex::new([max, 0.1, max], normal_top, grey),
                        Vertex::new([min, 0.1, min], normal_top, grey),
                        Vertex::new([max, 0.1, max], normal_top, grey),
                        Vertex::new([max, 0.1, min], normal_top, grey),
                        // back
                        Vertex::new([min, 0.0, min], normal_back, grey),
                        Vertex::new([min, 0.1, min], normal_back, grey),
                        Vertex::new([max, 0.0, min], normal_back, grey),
                        Vertex::new([max, 0.0, min], normal_back, grey),
                        Vertex::new([min, 0.1, min], normal_back, grey),
                        Vertex::new([max, 0.1, min], normal_back, grey),
                        // left
                        Vertex::new([min, 0.0, min], normal_left, grey),
                        Vertex::new([min, 0.0, max], normal_left, grey),
                        Vertex::new([min, 0.1, min], normal_left, grey),
                        Vertex::new([min, 0.1, min], normal_left, grey),
                        Vertex::new([min, 0.0, max], normal_left, grey),
                        Vertex::new([min, 0.1, max], normal_left, grey),
                        // right
                        Vertex::new([max, 0.0, min], normal_right, grey),
                        Vertex::new([max, 0.1, min], normal_right, grey),
                        Vertex::new([max, 0.0, max], normal_right, grey),
                        Vertex::new([max, 0.1, min], normal_right, grey),
                        Vertex::new([max, 0.1, max], normal_right, grey),
                        Vertex::new([max, 0.0, max], normal_right, grey),
                    ]);
                }
                TileConfiguration::All => {
                    vertices.extend_from_slice(&[
                        // ground plane
                        Vertex::new([min, 0.1, min], normal_top, grey),
                        Vertex::new([min, 0.1, max], normal_top, grey),
                        Vertex::new([max, 0.1, max], normal_top, grey),
                        Vertex::new([min, 0.1, min], normal_top, grey),
                        Vertex::new([max, 0.1, max], normal_top, grey),
                        Vertex::new([max, 0.1, min], normal_top, grey),
                        // front
                        Vertex::new([min, 0.0, max], normal_front, grey),
                        Vertex::new([max, 0.0, max], normal_front, grey),
                        Vertex::new([min, 0.1, max], normal_front, grey),
                        Vertex::new([max, 0.0, max], normal_front, grey),
                        Vertex::new([max, 0.1, max], normal_front, grey),
                        Vertex::new([min, 0.1, max], normal_front, grey),
                        // back
                        Vertex::new([min, 0.0, min], normal_back, grey),
                        Vertex::new([min, 0.1, min], normal_back, grey),
                        Vertex::new([max, 0.0, min], normal_back, grey),
                        Vertex::new([max, 0.0, min], normal_back, grey),
                        Vertex::new([min, 0.1, min], normal_back, grey),
                        Vertex::new([max, 0.1, min], normal_back, grey),
                        // left
                        Vertex::new([min, 0.0, min], normal_left, grey),
                        Vertex::new([min, 0.0, max], normal_left, grey),
                        Vertex::new([min, 0.1, min], normal_left, grey),
                        Vertex::new([min, 0.1, min], normal_left, grey),
                        Vertex::new([min, 0.0, max], normal_left, grey),
                        Vertex::new([min, 0.1, max], normal_left, grey),
                        // right
                        Vertex::new([max, 0.0, min], normal_right, grey),
                        Vertex::new([max, 0.1, min], normal_right, grey),
                        Vertex::new([max, 0.0, max], normal_right, grey),
                        Vertex::new([max, 0.1, min], normal_right, grey),
                        Vertex::new([max, 0.1, max], normal_right, grey),
                        Vertex::new([max, 0.0, max], normal_right, grey),
                    ]);
                }
            },
            TileType::Test => assert!(false),
        }
        Self {
            vertices,
            just_loaded: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mesh::triangle_normal;

    #[test]
    fn check_understanding_normal_calculation_0() {
        // counter clockwise triangle
        let p00 = [0.0, 0.0, 0.0];
        let p01 = [0.0, 0.0, 1.0];
        let p10 = [1.0, 0.0, 0.0];
        let normalized_normal: [f32; 3] = triangle_normal(p00, p01, p10);
        assert_eq!([0.0, 1.0, 0.0], normalized_normal);
    }
    #[test]
    fn check_understanding_normal_calculation_1() {
        // counter clockwise triangle
        let p00 = [-1.0, 0.0, -1.0];
        let p01 = [0.0, 0.0, 0.0];
        let p10 = [0.0, 0.0, -1.0];
        let normalized_normal: [f32; 3] = triangle_normal(p00, p01, p10);
        assert_eq!([0.0, 1.0, 0.0], normalized_normal);
    }
}
