use software_renderer_rs::*;
use std::fs::File;
use std::io::BufReader;
use obj::*;
use nalgebra::{Vector2, Vector3, Vector4, Matrix, U1};
use std::time::{Duration, Instant};

struct Vertex {
    pub v: Vector3<f32>,
    pub n: Vector3<f32>,
}

fn move_and_scale(v: &Vector3<f32>, m: f32, s_x: f32, s_y: f32) -> Vector3<f32> {
    Vector3::new((v.x + m) * s_x, (v.y + m) * s_y, v.z)
}

pub trait Shader {
    fn vertex(&self, in_vertex: &Vector3<f32>) -> Vector3<f32>;
    fn fragment(&self, in_fragment: &Vector2<f32>, in_color: &Color) -> Option<Color>;
}

struct BasicShader;

impl Shader for BasicShader {
    fn vertex(&self, in_vertex: &Vector3<f32>) -> Vector3<f32> {
        move_and_scale(&in_vertex, 1.0, 400.0, 400.0)
    }
    fn fragment(&self, in_fragment: &Vector2<f32>, in_color: &Color) -> Option<Color> {
        unimplemented!()
    }
}

fn draw_line(v0: Vector3<f32>, v1: Vector3<f32>, color: &Color, canvas: &mut Canvas) {
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
            canvas.set(y as usize, x as usize, color);
        }
    } else {
        for x in x0 as i32..x1 as i32 + 1 {
            let t: f32 = (x as f32 - x0) / (x1 - x0);
            let y = y0 * (1.0 - t) + y1 * t;
            canvas.set(x as usize, y as usize, color);
        }
    }
}

fn barycentric(v0: &Vector2<i32>, v1: &Vector2<i32>, v2: &Vector2<i32>, p: Vector2<i32>) -> Vector3<f32> {
    let x_vec: Vector3<i32> = Vector3::new(v2.x - v0.x, v1.x - v0.x, v0.x - p.x);
    let y_vec: Vector3<i32> = Vector3::new(v2.y - v0.y, v1.y - v0.y, v0.y - p.y);
    let u: Vector3<i32> = nalgebra::Vector3::cross(&x_vec, &y_vec);
    if u.z.abs() < 1 {
        return Vector3::new(-1.0, 1.0, 1.0);
    }
    return Vector3::new(1.0 - (u.x as f32 + u.y as f32) / u.z as f32, u.y as f32 / u.z as f32, u.x as f32 / u.z as f32);
}

fn draw_triangle(v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>, color: &Color, mut canvas: &mut Canvas, zbuffer: &mut Vec<f32>, width: usize, height: usize) {
    let v0i: Vector2<i32> = Vector2::new(v0.x as i32, v0.y as i32);
    let v1i: Vector2<i32> = Vector2::new(v1.x as i32, v1.y as i32);
    let v2i: Vector2<i32> = Vector2::new(v2.x as i32, v2.y as i32);
    let x_min = std::cmp::max(0, std::cmp::min(v0i.x, std::cmp::min(v1i.x, v2i.x)));
    let x_max = std::cmp::min(width as i32, std::cmp::max(v0i.x, std::cmp::max(v1i.x, v2i.x)));
    let y_min = std::cmp::max(0, std::cmp::min(v0i.y, std::cmp::min(v1i.y, v2i.y)));
    let y_max = std::cmp::min(height as i32, std::cmp::max(v0i.y, std::cmp::max(v1i.y, v2i.y)));
    for x in x_min..x_max {
        for y in y_min..y_max {
            let barycentric_screen = barycentric(&v0i, &v1i, &v2i, Vector2::new(x, y));
            if barycentric_screen.x >= 0.0 && barycentric_screen.y >= 0.0 && barycentric_screen.z >= 0.0 {
                let mut z: f32 = barycentric_screen.x * v0.z + barycentric_screen.y * v1.z + barycentric_screen.z * v2.z;
                if zbuffer[x as usize + width * y as usize] < z {
                    zbuffer[x as usize + width * y as usize] = z;
                    canvas.set(x as usize, y as usize, color);
                }
            }
        }
    }
}

fn render_model(shader: &Shader, model: &Vec<[Vector3<f32>; 3]>, width: usize, height: usize, mut canvas: &mut Canvas, mut zbuffer: &mut Vec<f32>) {
    let c = &Color {r: 255, g: 255, b: 255, a: 255 };
    let c_lines = &Color {r: 0, g: 0, b: 255, a: 255 };
    let mut triangle_count: i32 = 0;
    for t in model {
        let half_width = width as f32 / 2.0;
        let half_height = height as f32 / 2.0;
        let p0 = shader.vertex(&t[0]);
        let p1 = shader.vertex(&t[1]);
        let p2 = shader.vertex(&t[2]);
        triangle_count = triangle_count + 1;

        //draw_line(p0, p1, c_lines, &mut canvas);
        //draw_line(p1, p2, c_lines, &mut canvas);
        //draw_line(p2, p0, c_lines, &mut canvas);

        let light_direction: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);
        let n: Vector3<f32> = nalgebra::Vector3::cross(&(t[2] - t[0]), &(t[1] - t[0]));
        let n: Vector3<f32> = n.normalize();
        let intensity: f32 = n.dot(&light_direction);

        if intensity > 0.0 {
            let c_intensity = &Color { r: (c.r as f32 * intensity) as u8, g: (c.g as f32 * intensity) as u8, b: (c.b as f32 * intensity) as u8, a: c.a};
            draw_triangle(p0, p1, p2, &c_intensity, &mut canvas, &mut zbuffer, width, height);
        }
    }
    println!("triangle_count: {}", triangle_count)
}

fn main() -> Result<(), ObjError> {
    let width: usize = 800;
    let height: usize = 800;
    let mut canvas = Canvas::new(width, height, Color{r: 0, g:0, b: 0, a: 255});
    let mut zbuffer: Vec<f32> = vec![0.0; width * height];
    let window: Window = Window::new(&canvas);
    let shader = BasicShader;

    let input = BufReader::new(File::open("/Users/bjornmartens/projects/tempfromgithub/tinyrenderer/obj/african_head/african_head.obj")?);
    let model_obj: Obj = load_obj(input)?;

    let model : &mut Vec<[Vector3<f32>; 3]>= &mut Vec::new();
    for indices in model_obj.indices.chunks(3) {
        let first = model_obj.vertices[indices[0] as usize];
        let second = model_obj.vertices[indices[1] as usize];
        let third = model_obj.vertices[indices[2] as usize];
        model.push([Vector3::new(first.position[0], first.position[1], first.position[2]),
                                Vector3::new(second.position[0], second.position[1], second.position[2]),
                                Vector3::new(third.position[0], third.position[1], third.position[2])]);
    }
    let now = Instant::now();

    let mut previous_time = Instant::now();
    while window.pump() {
        render_model(&shader, &model, width, height, &mut canvas, &mut zbuffer);
        let current_time = Instant::now();
        println!("fps: {}", (current_time - previous_time).as_millis());
        previous_time = current_time;
        window.update();
    }
    Ok(())
}
