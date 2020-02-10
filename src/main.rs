use software_renderer_rs::*;
use std::fs::File;
use std::io::BufReader;
use obj::*;
use nalgebra::{Vector2, Vector3};
use std::time::{Duration, Instant};

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

fn draw_triangle_2d(v0: Vector2<i32>, v1: Vector2<i32>, v2: Vector2<i32>, color: &Color, canvas: &mut Canvas, width: usize, height: usize) {
    let x_min = std::cmp::max(0, std::cmp::min(v0.x, std::cmp::min(v1.x, v2.x)));
    let x_max = std::cmp::min(width as i32, std::cmp::max(v0.x, std::cmp::max(v1.x, v2.x)));
    let y_min = std::cmp::max(0, std::cmp::min(v0.y, std::cmp::min(v1.y, v2.y)));
    let y_max = std::cmp::min(height as i32, std::cmp::max(v0.y, std::cmp::max(v1.y, v2.y)));
    for x in x_min..x_max {
        for y in y_min..y_max {
            let barycentric_screen = barycentric(&v0, &v1, &v2, Vector2::new(x, y));
            if barycentric_screen.x >= 0.0 && barycentric_screen.y >= 0.0 && barycentric_screen.z >= 0.0 {
                canvas.set(x as usize, y as usize, color);
            }
        }
    }
}

fn draw_triangle(v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>, color: &Color, mut canvas: &mut Canvas, width: usize, height: usize) {
    draw_triangle_2d(Vector2::new(v0.x as i32, v0.y as i32), Vector2::new(v1.x as i32, v1.y as i32),
                     Vector2::new(v2.x as i32, v2.y as i32), &color, &mut canvas, width, height);
}

fn move_and_scale(v: Vector3<f32>, m: f32, s_x: f32, s_y: f32) -> Vector3<f32> {
    Vector3::new((v.x + m) * s_x, (v.y + m) * s_y, v.z)
}

fn render_model(model: &Vec<[Vector3<f32>; 3]>, width: usize, height: usize, mut canvas: &mut Canvas) {
    let c = &Color {r: 255, g: 255, b: 255, a: 255 };
    let c_lines = &Color {r: 0, g: 0, b: 255, a: 255 };
    let mut triangle_count: i32 = 0;
    let light_direction: Vector3<f32> = Vector3::new(0.0, 0.0, -1.0);
    for t in model {
        let half_width = width as f32 / 2.0;
        let half_height = height as f32 / 2.0;
        let p0 = move_and_scale(t[0], 1.0, half_width, half_height);
        let p1 = move_and_scale(t[1], 1.0, half_width, half_height);
        let p2 = move_and_scale(t[2], 1.0, half_width, half_height);
        triangle_count = triangle_count + 1;

        //draw_line(p0, p1, c_lines, &mut canvas);
        //draw_line(p1, p2, c_lines, &mut canvas);
        //draw_line(p2, p0, c_lines, &mut canvas);

        let n: Vector3<f32> = nalgebra::Vector3::cross(&(t[2] - t[0]), &(t[1] - t[0]));
        let n: Vector3<f32> = n.normalize();
        let intensity: f32 = n.dot(&light_direction);

        if intensity > 0.0 {
            let c_intensity = &Color { r: (c.r as f32 * intensity) as u8, g: (c.g as f32 * intensity) as u8, b: (c.b as f32 * intensity) as u8, a: c.a};
            draw_triangle(p0, p1, p2, &c_intensity, &mut canvas, width, height);
        }
    }
    println!("triangle_count: {}", triangle_count)
}

fn main() -> Result<(), ObjError> {
    let width: usize = 800;
    let height: usize = 800;
    let mut canvas = Canvas::new(width, height, Color{r: 0, g:0, b: 0, a: 255});
    let window: Window = Window::new(&canvas);

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
        render_model(&model, width, height, &mut canvas);
        let current_time = Instant::now();
        println!("fps: {}", (current_time - previous_time).as_millis());
        previous_time = current_time;
        window.update();
    }
    Ok(())
}
