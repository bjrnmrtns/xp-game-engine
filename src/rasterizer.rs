use crate::vec::{Vec3, Vec4, Vec2};
use software_renderer_rs::{Color, Canvas};

pub trait Vary {
    fn vary(var0: &Self, var1: &Self, var2: &Self, bc: &Vec3<f32>) -> Self;
}

pub trait Shader<V: Vary> {
    fn vertex(&self, in_v: &Vec3<f32>, var: &V) -> (Vec4<f32>, V);
    fn fragment(&self, in_f: &Vec2<f32>, var: V) -> Option<Color>;
}

fn barycentric(v0: &Vec2<i32>, v1: &Vec2<i32>, v2: &Vec2<i32>, p: Vec2<i32>) -> Vec3<f32> {
    let x_vec: Vec3<i32> = Vec3::new(v2.x - v0.x, v1.x - v0.x, v0.x - p.x);
    let y_vec: Vec3<i32> = Vec3::new(v2.y - v0.y, v1.y - v0.y, v0.y - p.y);
    let u: Vec3<i32> = x_vec.cross(y_vec);
    if u.z.abs() < 1 {
        return Vec3::new(-1.0, 1.0, 1.0);
    }
    return Vec3::new(1.0 - (u.x as f32 + u.y as f32) / u.z as f32, u.y as f32 / u.z as f32, u.x as f32 / u.z as f32);
}

fn bounding_box<T: std::cmp::Ord + Copy>(points: &[Vec2<T>]) -> (Vec2<T>, Vec2<T>) {
    let mut min = points[0];
    let mut max = points[0];
    for p in points {
        min.x = std::cmp::min(min.x, p.x);
        min.y = std::cmp::min(min.y, p.y);
        max.x = std::cmp::max(max.x, p.x);
        max.y = std::cmp::max(max.y, p.y);
    }
    (min, max)
}

pub(crate) fn draw_triangle<V: Vary, S: Shader<V>>(v0: (Vec3<f32>, V), v1: (Vec3<f32>, V), v2: (Vec3<f32>, V), shader: &S, canvas: &mut Canvas) {
    let p0 = shader.vertex(&v0.0, &v0.1);
    let p1 = shader.vertex(&v1.0, &v1.1);
    let p2 = shader.vertex(&v2.0, &v2.1);
    let xy0 = p0.0.xy() / p0.0.w;
    let xy1 = p1.0.xy() / p1.0.w;
    let xy2 = p2.0.xy() / p2.0.w;
    let v0i: Vec2<i32> = Vec2::new(xy0.x as i32, xy0.y as i32);
    let v1i: Vec2<i32> = Vec2::new(xy1.x as i32, xy1.y as i32);
    let v2i: Vec2<i32> = Vec2::new(xy2.x as i32, xy2.y as i32);
    let (min, max) = bounding_box(&[v0i, v1i, v2i]);
    for x in min.x..max.x {
        for y in min.y..max.y {
            let bs = barycentric(&v0i, &v1i, &v2i, Vec2::new(x, y));
            if bs.x >= 0.0 && bs.y >= 0.0 && bs.z >= 0.0 {
                //w' = ( 1 / v0.v.w ) * bs.x + ( 1 / v1.v.w ) * bs.y + ( 1 / v2.v.w ) * bs.z
                //u' = ( v0.t.u / v0.t.w ) * bs.x + ( v1.t.u / v1.t.w ) * bs.y + ( v2.t.u / v2.t.w ) * bs.z
                //v' = ( v0.t.v / v0.t.w ) * bs.x + ( v1.t.v / v1.t.w ) * bs.y + ( v2.t.v / v2.t.w ) * bs.z
                //perspCorrU = u' / w'
                //perspCorrV = v' / w'

                let depth: f32 = bs.x * v0.0.z + bs.y * v1.0.z + bs.z * v2.0.z;
                match shader.fragment(&Vec2::new(x as f32, y as f32), V::vary(&v0.1, &v1.1, &v2.1, &bs)) {
                    Some(c) => canvas.set_with_depth(x as usize, y as usize, depth, &c),
                    None => (),
                }
            }
        }
    }
}
