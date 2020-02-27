use software_renderer_rs::*;
use std::fs::File;
use std::io::BufReader;
use std::time::{Instant};

mod vec;
use vec::{Vec2, Vec3, Vec4};
mod mat;
use mat::{Mat4};
use image::RgbImage;
use crate::rasterizer::{Vary, Shader, draw_triangle};

mod obj;

mod rasterizer {
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
}


#[derive(Copy, Clone)]
pub struct Varyings {
    n: Vec3<f32>,
    t: Vec2<f32>,
}

fn viewport(x: f32, y: f32, width: f32, height: f32, depth: f32) -> Mat4<f32> {
    Mat4(
        width / 2.0, 0.0, 0.0, x + width / 2.0,
        0.0, height / 2.0, 0.0, y + height / 2.0,
        0.0, 0.0, depth / 2.0, depth / 2.0,
        0.0, 0.0 , 0.0, 1.0)
}

impl Vary for Varyings {
    fn vary(var0: &Self, var1: &Self, var2: &Self, bc: &Vec3<f32>) -> Self {
        Varyings {
            n: (var0.n * bc.x + var1.n * bc.y + var2.n * bc.z).normalize(),
            t: (var0.t * bc.x + var1.t * bc.y + var2.t * bc.z),
        }
    }
}

struct BasicShader<'a>
{
    mat: &'a Mat4<f32>,
    tex: &'a RgbImage,
    light_direction: Vec3<f32>,
}

impl<'a> Shader<Varyings> for BasicShader<'a> {
    fn vertex(&self, in_v: &Vec3<f32>, var: &Varyings) -> (Vec4<f32>, Varyings) {
        (*self.mat * Vec4::new(in_v.x, in_v.y, in_v.z, 1.0),
         *var)
    }
    fn fragment(&self, _: &Vec2<f32>, var: Varyings) -> Option<Color> {
        let intensity = var.n.dot(self.light_direction);
        if intensity > 0.0 {
            let pixel = self.tex.get_pixel((var.t.x * self.tex.width() as f32) as u32, self.tex.height() - 1 - (var.t.y * self.tex.height() as f32) as u32);
            let r = pixel[0] as f32 * intensity;
            let g = pixel[1] as f32 * intensity;
            let b = pixel[2] as f32 * intensity;
            let out_color = Color { r: r as u8, g: g as u8, b: b as u8, a: 255 };
            Some(out_color)
        } else {
            None
        }
    }
}

fn _load_triangle() -> obj::ObjResult<Vec<[(Vec3<f32>, Varyings); 3]>> {
    let first: Vec3<f32> = Vec3::new(1.0, 0.0, 0.0);
    let second: Vec3<f32> = Vec3::new(0.0, 1.0, 1.0);
    let third: Vec3<f32> = Vec3::new(-1.0, 0.0, 0.0);
    let n: Vec3<f32> = (third - first).cross(second - first);
    let mut triangle = Vec::new();
    triangle.push([ (first, Varyings { n, t: Vec2::new(1.0, 0.0)}),
                           (second, Varyings { n, t: Vec2::new(0.5, 1.0)}),
                          (third, Varyings{ n, t: Vec2::new(0.0, 0.0)})]);
    Ok(triangle)
}

pub fn load_mesh<R>(reader: R) -> obj::ObjResult<Vec<[(Vec3<f32>, Varyings); 3]>>
    where R: std::io::BufRead {
    let vertices = obj::parse_obj(reader)?;
    let mut result = Vec::new();
    for v in vertices {
        result.push([(v[0].0, Varyings { n: (v[0].1).0, t: (v[0].1).1} ),
                            (v[1].0, Varyings { n: (v[1].1).0, t: (v[1].1).1} ),
                            (v[2].0, Varyings { n: (v[2].1).0, t: (v[2].1).1 })]);
    }
    Ok(result)
}


fn main() -> std::result::Result<(), obj::ObjError> {
    let width: usize = 800;
    let height: usize = 800;
    let img: RgbImage = image::open("/Users/bjornmartens/projects/software-renderer-rs/obj/ah/african_head_diffuse.tga").unwrap().to_rgb(); // use try/? but convert to generic error to standard error and change result of main into that error.
    let mat = viewport(0.0, 0.0, 800.0, 800.0, 255.0);
    let shader = BasicShader {
        mat: &mat,
        tex: &img,
        light_direction: Vec3::new(0.0, 0.0, 1.0),
    };
    let mut canvas = Canvas::new(width, height, Color{r: 0, g:0, b: 0, a: 255});
    let window: Window = Window::new(&canvas);

    let input = &mut BufReader::new(File::open("/Users/bjornmartens/projects/software-renderer-rs/obj/ah/african_head.obj")?);
    let model = load_mesh(input)?;
    let mut previous_time = Instant::now();
    while window.pump() {
        let mut triangle_count: i32 = 0;
        for t in &model {
            triangle_count = triangle_count + 1;
            draw_triangle(t[0], t[1], t[2], &shader, &mut canvas);
        }
        println!("triangle_count: {}", triangle_count);
        let current_time = Instant::now();
        println!("fps: {}", (current_time - previous_time).as_millis());
        previous_time = current_time;
        window.update();
        canvas.clear_zbuffer();
    }
    Ok(())
}


