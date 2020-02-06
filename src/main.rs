use software_renderer_rs::*;
use std::fs::File;
use std::io::BufReader;
use obj::*;
use nalgebra::{Vector3};

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

/*
fn barycentric(v0: Vector2<f32>, v1: Vector2<f32>, v2: Vector2<f32>) -> Vector3<f32> {
}
*/
fn draw_triangle(v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>, color: &Color, canvas: &mut Canvas) {
}


fn move_and_scale(v: Vector3<f32>, m: f32, s_x: f32, s_y: f32) -> Vector3<f32> {
    Vector3::new((v.x + m) * s_x, (v.y + m) * s_y, v.z)
}

fn main() -> Result<(), ObjError> {
    let width: usize = 800;
    let height: usize = 800;
    let mut canvas = Canvas::new(width, height, Color{r: 0, g:0, b: 0, a: 255});
    let window: Window = Window::new(&canvas);

    let input = BufReader::new(File::open("african_head.obj")?);
    let model: Obj = load_obj(input)?;

    let mut vertex_data : Vec<[Vector3<f32>; 3]>= Vec::new();
    for indices in model.indices.chunks(3) {
        let first = model.vertices[indices[0] as usize];
        let second = model.vertices[indices[1] as usize];
        let third = model.vertices[indices[2] as usize];
        vertex_data.push([Vector3::new(first.position[0], first.position[1], first.position[2]),
                                Vector3::new(second.position[0], second.position[1], second.position[2]),
                                Vector3::new(third.position[0], third.position[1], third.position[2])]);
    }

    let c = &Color {r: 255, g: 255, b: 255, a: 255 };
    for t in vertex_data {
        let half_width = width as f32 / 2.0;
        let half_height = height as f32 / 2.0;
        let p0 = move_and_scale(t[0], 1.0, half_width, half_height);
        let p1 = move_and_scale(t[1], 1.0, half_width, half_height);
        let p2 = move_and_scale(t[2], 1.0, half_width, half_height);
        draw_line(p0, p1, c, &mut canvas);
        draw_line(p1, p2, c, &mut canvas);
        draw_line(p2, p0, c, &mut canvas);

        draw_triangle(p0, p1, p2, c, &mut canvas);
    }

    while window.pump() {
        window.update();
    }
    Ok(())
}
