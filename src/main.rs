use software_renderer_rs::*;
use std::fs::File;
use std::io::BufReader;
use obj::*;
use std::time::{Duration, Instant};

mod vec;
use vec::{Vec2, Vec3, Vec4};
mod mat;
use mat::{Mat4};

pub struct Vertex {
    pub v: Vec3<f32>,
    pub n: Vec3<f32>,
    pub t: Vec2<f32>,
}

pub struct VertexOut {
    pub v: Vec4<f32>,
    pub n: Vec3<f32>,
    pub t: Vec2<f32>,
}

fn viewport(x: i32, y: i32, width: i32, height: i32, depth: i32) -> Mat4<f32> {
    Mat4(
        width as f32 / 2.0, 0.0, 0.0, x as f32 + width as f32 / 2.0,
        0.0, height as f32 / 2.0, 0.0, y as f32 + height as f32 / 2.0,
        0.0, 0.0, depth as f32 / 2.0, depth as f32 / 2.0,
        0.0, 0.0 , 0.0, 1.0)
}

pub trait Shader {
    fn vertex(&self, in_vertex: &Vertex, projection: &Mat4<f32>) -> VertexOut;
    fn fragment(&self, image: &image::RgbImage, in_fragment: &Vec2<f32>, in_texcoord: &Vec2<f32>, intensity: f32) -> Option<Color>;
}

struct BasicShader;

impl Shader for BasicShader {
    fn vertex(&self, in_vertex: &Vertex, projection: &Mat4<f32>) -> VertexOut {
        let r = viewport(0, 0, 800, 800, 255) * Vec4::new(in_vertex.v.x, in_vertex.v.y, in_vertex.v.z, 1.0);
        VertexOut { v: r, n: in_vertex.n.clone(), t: in_vertex.t.clone()  }
    }
    fn fragment(&self, image: &image::RgbImage, in_fragment: &Vec2<f32>, in_texcoord: &Vec2<f32>, intensity: f32) -> Option<Color> {
        let pixel = image.get_pixel((in_texcoord.x * image.width() as f32) as u32, image.height() - 1 - (in_texcoord.y * image.height() as f32) as u32);
        let r = pixel[0]; let g = pixel[1]; let b = pixel[2];
        let out_color = Color{ r: (r as f32 * intensity) as u8, g: (g as f32 * intensity) as u8, b: (b as f32 * intensity) as u8, a: 255 };
        Some(out_color)
    }
}

fn draw_line(v0: Vec3<f32>, v1: Vec3<f32>, color: Color, canvas: &mut Canvas) {
    let mut steep = false;
    let mut x0 = v0.x;
    let mut y0 = v0.y;
    let mut x1 = v1.x;
    let mut y1 = v1.y;
    if (v1.x - v0.x).abs() < (v0.y - v1.y).abs() {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }
    if steep {
        for x in x0 as i32..x1 as i32 + 1 {
            let t: f32 = (x as f32 - x0) / (x1 - x0);
            let y = y0 * (1.0 - t) + y1 * t;
            canvas.set(y as usize, x as usize, &color);
        }
    } else {
        for x in x0 as i32..x1 as i32 + 1 {
            let t: f32 = (x as f32 - x0) / (x1 - x0);
            let y = y0 * (1.0 - t) + y1 * t;
            canvas.set(x as usize, y as usize, &color);
        }
    }
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

fn draw_triangle(shader: &Shader, image: &image::RgbImage, v0: VertexOut, v1: VertexOut, v2: VertexOut, intensity: f32, canvas: &mut Canvas, zbuffer: &mut Vec<f32>, width: usize, height: usize) {
    let v0i: Vec2<i32> = Vec2::new(v0.v.x as i32, v0.v.y as i32);
    let v1i: Vec2<i32> = Vec2::new(v1.v.x as i32, v1.v.y as i32);
    let v2i: Vec2<i32> = Vec2::new(v2.v.x as i32, v2.v.y as i32);
    let x_min = std::cmp::max(0, std::cmp::min(v0i.x, std::cmp::min(v1i.x, v2i.x)));
    let x_max = std::cmp::min(width as i32, std::cmp::max(v0i.x, std::cmp::max(v1i.x, v2i.x)));
    let y_min = std::cmp::max(0, std::cmp::min(v0i.y, std::cmp::min(v1i.y, v2i.y)));
    let y_max = std::cmp::min(height as i32, std::cmp::max(v0i.y, std::cmp::max(v1i.y, v2i.y)));
    for x in x_min..x_max {
        for y in y_min..y_max {
            let bs = barycentric(&v0i, &v1i, &v2i, Vec2::new(x, y));
            if bs.x >= 0.0 && bs.y >= 0.0 && bs.z >= 0.0 {
                //w' = ( 1 / v0.v.w ) * bs.x + ( 1 / v1.v.w ) * bs.y + ( 1 / v2.v.w ) * bs.z
                //u' = ( v0.t.u / v0.t.w ) * bs.x + ( v1.t.u / v1.t.w ) * bs.y + ( v2.t.u / v2.t.w ) * bs.z
                //v' = ( v0.t.v / v0.t.w ) * bs.x + ( v1.t.v / v1.t.w ) * bs.y + ( v2.t.v / v2.t.w ) * bs.z
                //perspCorrU = u' / w'
                //perspCorrV = v' / w'
                let u = bs.x * v0.t.x + bs.y * v1.t.x + bs.z * v2.t.x;
                let v = bs.x * v0.t.y + bs.y * v1.t.y + bs.z * v2.t.y;

                let depth: f32 = bs.x * v0.v.z + bs.y * v1.v.z + bs.z * v2.v.z;
                match shader.fragment(image, &Vec2::new(x as f32, y as f32), &Vec2::new(u, v), intensity) {
                    Some(c) => canvas.set_with_depth(x as usize, y as usize, depth as isize, &c),
                    None => (),
                }
            }
        }
    }
}
fn load_triangle() -> Vec<[Vertex; 3]> {
    let first: Vec3<f32> = Vec3::new(1.0, 0.0, 0.0);
    let second: Vec3<f32> = Vec3::new(0.0, 1.0, 1.0);
    let third: Vec3<f32> = Vec3::new(-1.0, 0.0, 0.0);
    let n: Vec3<f32> = (third - first).cross(second - first);
    let t: Vec2<f32> = Vec2::new(0.0, 0.0);
    let mut triangle : Vec<[Vertex; 3]>= Vec::new();
    triangle.push([ Vertex {v: first, n: n, t: Vec2::new(1.0, 0.0)},
                          Vertex{v: second, n: n, t: Vec2::new(0.5, 1.0)},
                          Vertex{v: third, n: n, t: Vec2::new(0.0, 0.0)}]);
    triangle
}

fn load_model<R: std::io::BufRead>(r: R) -> Result<Vec<[Vertex; 3]>, ObjError> {
    let model_obj: Obj<TexturedVertex> = load_obj(r)?;
    let mut model : Vec<[Vertex; 3]>= Vec::new();
    for indices in model_obj.indices.chunks(3) {
        let first = model_obj.vertices[indices[0] as usize];
        let second = model_obj.vertices[indices[1] as usize];
        let third = model_obj.vertices[indices[2] as usize];
        model.push([ Vertex{ v: Vec3::new(first.position[0], first.position[1], first.position[2]), n:  Vec3::new(first.normal[0], first.normal[1], first.normal[2]), t: Vec2::new(first.texture[0], first.texture[1]) },
                            Vertex{ v: Vec3::new(second.position[0], second.position[1], second.position[2]), n: Vec3::new(second.normal[0], second.normal[1], second.normal[2]), t: Vec2::new(second.texture[0], second.texture[1]) },
                            Vertex{ v: Vec3::new(third.position[0], third.position[1], third.position[2]), n: Vec3::new(third.normal[0], third.normal[1], third.normal[2]), t: Vec2::new(third.texture[0], third.texture[1]) }]);
    }
	Ok(model)
}

fn render_model(shader: &Shader, image: &image::RgbImage, model: &[[Vertex; 3]], width: usize, height: usize, mut canvas: &mut Canvas, mut zbuffer: &mut Vec<f32>) {
    let c = &Color {r: 255, g: 255, b: 255, a: 255 };
    let c_lines = &Color {r: 0, g: 0, b: 255, a: 255 };
    let projection = Mat4(1.0, 0.0, 0.0, 0.0,
                          0.0, 1.0, 0.0, 0.0,
                          0.0, 0.0, 1.0, 0.0,
                          0.0, 0.0, -0.33, 1.0);
    let mut triangle_count: i32 = 0;
    for t in model {
        let p0 = shader.vertex(&t[0], &projection);
        let p1 = shader.vertex(&t[1], &projection);
        let p2 = shader.vertex(&t[2], &projection);
        triangle_count = triangle_count + 1;

        let light_direction: Vec3<f32> = Vec3::new(0.0, 0.0, -1.0);
        let n: Vec3<f32> = (t[2].v - t[0].v).cross(t[1].v - t[0].v);
        let n: Vec3<f32> = n.normalize();
        let intensity: f32 = n.dot(light_direction);

        if intensity > 0.0 {
            draw_triangle(shader, image, p0, p1, p2, intensity, &mut canvas, &mut zbuffer, width, height);
        }
    }
    println!("triangle_count: {}", triangle_count)
}

fn main() -> Result<(), ObjError> {
    let width: usize = 800;
    let height: usize = 800;
    let img = image::open("/Users/bjornmartens/projects/software-renderer-rs/obj/ah/african_head_diffuse.tga").unwrap().to_rgb(); // use try/? but convert to generic error to standard error and change result of main into that error.
    let mut canvas = Canvas::new(width, height, Color{r: 0, g:0, b: 0, a: 255});
    let mut zbuffer: &mut Vec<f32> = &mut vec![0.0; width * height];
    let window: Window = Window::new(&canvas);
    let shader = BasicShader;

    let input = &mut BufReader::new(File::open("/Users/bjornmartens/projects/software-renderer-rs/obj/ah/african_head.obj")?);
	let model = load_model(input)?;
    //let model = load_triangle();
    let mut previous_time = Instant::now();
    while window.pump() {
        render_model(&shader, &img, &model, width, height, &mut canvas, &mut zbuffer);
        let current_time = Instant::now();
        println!("fps: {}", (current_time - previous_time).as_millis());
        previous_time = current_time;
        window.update();
    }
    Ok(())
}


