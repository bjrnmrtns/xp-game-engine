use crate::canvas::{Canvas, Color};
use nalgebra_glm::*;

pub trait Vary {
    fn vary(var0: &Self, var1: &Self, var2: &Self, bc: &Vec3) -> Self;
}

pub trait Shader<V: Vary> {
    fn vertex(&self, in_v: &Vec3, var: &V) -> (Vec4, V);
    fn fragment(&self, in_f: &I32Vec2, var: &V) -> Option<Color>;
}

fn barycentric(v0: &I32Vec2, v1: &I32Vec2, v2: &I32Vec2, p: I32Vec2) -> Vec3 {
    let x_vec: I32Vec3 = vec3(v2.x - v0.x, v1.x - v0.x, v0.x - p.x);
    let y_vec: I32Vec3 = vec3(v2.y - v0.y, v1.y - v0.y, v0.y - p.y);
    let u = cross(&x_vec, &y_vec);
    if u.z.abs() < 1 {
        return vec3(-1.0, 1.0, 1.0);
    }
    return vec3(1.0 - (u.x as f32 + u.y as f32) / u.z as f32, u.y as f32 / u.z as f32, u.x as f32 / u.z as f32);
}

fn bounding_box(points: &[I32Vec2]) -> (I32Vec2, I32Vec2) {
    let mut min = points[0].clone();
    let mut max = points[0].clone();
    for p in points {
        min.x = std::cmp::min(min.x, p.x);
        min.y = std::cmp::min(min.y, p.y);
        max.x = std::cmp::max(max.x, p.x);
        max.y = std::cmp::max(max.y, p.y);
    }
    (min, max)
}

pub fn draw_triangle<V: Vary, S: Shader<V>>(v0: (Vec3, V), v1: (Vec3, V), v2: (Vec3, V), shader: &S, canvas: &mut Canvas) {
    let ss0 = shader.vertex(&v0.0, &v0.1);
    let ss1 = shader.vertex(&v1.0, &v1.1);
    let ss2 = shader.vertex(&v2.0, &v2.1);

    let xy0: I32Vec2  = vec2((ss0.0.x / ss0.0.w) as i32, (ss0.0.y / ss0.0.w) as i32);
    let xy1: I32Vec2  = vec2((ss1.0.x / ss1.0.w) as i32, (ss1.0.y / ss1.0.w) as i32);
    let xy2: I32Vec2  = vec2((ss2.0.x / ss2.0.w) as i32, (ss2.0.y / ss2.0.w) as i32);
    let (min, max) = bounding_box(&[xy0, xy1, xy2]);
    for x in std::cmp::max(0, min.x)..std::cmp::min(800, max.x) {
        for y in std::cmp::max(0, min.y)..std::cmp::min(800, max.y) {
            let bs = barycentric(&xy0, &xy1, &xy2, vec2(x, y));
            if bs.x >= 0.0 && bs.y >= 0.0 && bs.z >= 0.0 {
                let depth: f32 = bs.x * ss0.0.z + bs.y * ss1.0.z + bs.z * ss2.0.z;
                if depth < 1.0 && depth > -1.0 { // near far plane clipping
                    match shader.fragment(&vec2(x, y), &V::vary(&ss0.1, &ss1.1, &ss2.1, &bs)) {
                        Some(c) => canvas.set_with_depth(x as usize, y as usize, depth, &c),
                        None => (),
                    }
                }
            }
        }
    }
}
