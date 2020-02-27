use software_renderer_rs::*;
use std::fs::File;
use std::io::BufReader;
use std::time::{Instant};

mod vec;
use vec::{Vec2, Vec3, Vec4};
mod mat;
use mat::{Mat4};
use image::RgbImage;

mod obj;

mod rasterizer;
use rasterizer::{Vary, Shader};


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
            rasterizer::draw_triangle(t[0], t[1], t[2], &shader, &mut canvas);
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


