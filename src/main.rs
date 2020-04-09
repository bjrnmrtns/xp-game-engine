mod obj;
mod rasterizer;
mod windowing;

use rasterizer::{Vary, Shader};
use windowing::*;

use image::RgbImage;
use nalgebra_glm::*;

use std::fs::File;
use std::io::BufReader;
use std::time::{Instant};

#[derive(Copy, Clone)]
pub struct Varyings {
    n: Vec3,
    t: Vec2,
}

fn viewport(x: f32, y: f32, width: f32, height: f32, depth: f32) -> Mat4 {
    mat4(
        width / 2.0, 0.0, 0.0, x + width / 2.0,
        0.0, height / 2.0, 0.0, y + height / 2.0,
        0.0, 0.0, depth / 2.0, depth / 2.0,
        0.0, 0.0 , 0.0, 1.0)
}

impl Vary for Varyings {
    fn vary(var0: &Self, var1: &Self, var2: &Self, bc: &Vec3) -> Self {
        Varyings {
            n: (var0.n * bc.x + var1.n * bc.y + var2.n * bc.z).normalize(),
            t: (var0.t * bc.x + var1.t * bc.y + var2.t * bc.z),
        }
    }
}

struct BasicShader<'a>
{
    mat: &'a Mat4,
    tex: &'a RgbImage,
    light_direction: Vec3,
}

impl<'a> Shader<Varyings> for BasicShader<'a> {
    fn vertex(&self, in_v: &Vec3, var: &Varyings) -> (Vec4, Varyings) {
        (*self.mat * vec4(in_v.x, in_v.y, in_v.z, 1.0),
         *var)
    }
    fn fragment(&self, _: Vec2, var: Varyings) -> Option<Color> {
        let intensity = dot(&var.n, &self.light_direction);
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

fn _load_triangle() -> obj::ObjResult<Vec<[(Vec3, Varyings); 3]>> {
    let first = vec3(1.0, 0.0, 0.0);
    let second = vec3(0.0, 1.0, 1.0);
    let third = vec3(-1.0, 0.0, 0.0);
    let n = cross(&(third - first),&(second - first));
    let mut triangle = Vec::new();
    triangle.push([ (first, Varyings { n, t: Vec2::new(1.0, 0.0)}),
                           (second, Varyings { n, t: Vec2::new(0.5, 1.0)}),
                          (third, Varyings{ n, t: Vec2::new(0.0, 0.0)})]);
    Ok(triangle)
}

pub fn load_mesh<R>(reader: R) -> obj::ObjResult<Vec<[(Vec3, Varyings); 3]>>
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
    //configuration
    let width: usize = 800;
    let height: usize = 800;
    //create_window
    let mut canvas = Canvas::new(width, height, Color{r: 0, g:0, b: 0, a: 255});
    let window: Window = Window::new(&canvas);
    //load_resources
    let img: RgbImage = image::open("obj/ah/african_head_diffuse.tga").unwrap().to_rgb(); // use try/? but convert to generic error to standard error and change result of main into that error.
    let input = &mut BufReader::new(File::open("obj/ah/african_head.obj")?);
    let model = load_mesh(input)?;

    //render
    let view = look_at(&vec3(0.5, 0.5, 0.0), &vec3(0.0, 0.0, -2.0), &vec3(0.0, 1.0, 0.0));
    let viewport = viewport(0.0, 0.0, 800.0, 800.0, 255.0);
    let shader = BasicShader {
        mat: &(viewport * view),
        tex: &img,
        light_direction: vec3(0.0, 0.0, 1.0),
    };

    let mut previous_time = Instant::now();
    let mut quit = false;
    while !quit {
        let mut quit_polling = false;
        while !quit_polling && !quit {
            match window.poll_event() {
                Some(InputEvent::Quit) => quit = true,
                None => quit_polling = true,
                _ => (),
            }
        }
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


