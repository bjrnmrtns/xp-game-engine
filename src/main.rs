use software_renderer_rs::*;
use std::fs::File;
use std::io::BufReader;
use obj::*;
use nalgebra::*;

struct Vec2i {
    x: i32,
    y: i32,
}

fn draw_line(v0: Vec2i, v1: Vec2i, color: &Color, canvas: &mut Canvas) {
    let mut steep = false;
    let mut x0 = v0.x as f64;
    let mut y0 = v0.y as f64;
    let mut x1 = v1.x as f64;
    let mut y1 = v1.y as f64;
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
            let t: f64 = (x as f64 - x0) / (x1 - x0);
            let y = y0 * (1.0 - t) + y1 * t;
            canvas.set(y as usize, x as usize, color);
        }
    } else {
        for x in x0 as i32..x1 as i32 + 1 {
            let t: f64 = (x as f64 - x0) / (x1 - x0);
            let y = y0 * (1.0 - t) + y1 * t;
            canvas.set(x as usize, y as usize, color);
        }
    }
}
/*
fn barycentric(v0: I32Vec2, v1: I32Vec2, v2: I32Vec2) -> I32Vec3 {
}

fn draw_triangle(v0: I32Vec2, v1: I32Vec2, v2: I32Vec2, color: &Color, canvas: &mut Canvas) {
}
*/
fn main() -> Result<(), ObjError> {
    let width: usize = 800;
    let height: usize = 800;
    let mut canvas = Canvas::new(width, height, Color{r: 0, g:0, b: 0, a: 255});
    let window: Window = Window::new(&canvas);

    let input = BufReader::new(File::open("african_head.obj")?);
    let model: Obj = load_obj(input)?;

    let mut triangles : Vec<[[f32; 3]; 3]> = Vec::new();
    for indices in model.indices.chunks(3) {
        let first = model.vertices[indices[0] as usize];
        let second = model.vertices[indices[1] as usize];
        let third = model.vertices[indices[2] as usize];
        triangles.push([first.position, second.position, third.position]);
    }

    for t in triangles {
        draw_line(Vec2i {x: ((t[0][0] + 1.0) * 400 as f32) as i32, y: ((t[0][1] + 1.0) * 400 as f32) as i32},
                  Vec2i {x: ((t[1][0] + 1.0) * 400 as f32) as i32, y: ((t[1][1] + 1.0) * 400 as f32) as i32 }, &Color {r: 255, g: 255, b: 255, a: 255 }, &mut canvas);
        draw_line(Vec2i {x: ((t[1][0] + 1.0) * 400 as f32) as i32, y: ((t[1][1] + 1.0) * 400 as f32) as i32},
                  Vec2i {x: ((t[2][0] + 1.0) * 400 as f32) as i32, y: ((t[2][1] + 1.0) * 400 as f32) as i32 }, &Color {r: 255, g: 255, b: 255, a: 255 }, &mut canvas);
        draw_line(Vec2i {x: ((t[2][0] + 1.0) * 400 as f32) as i32, y: ((t[2][1] + 1.0) * 400 as f32) as i32},
                  Vec2i {x: ((t[0][0] + 1.0) * 400 as f32) as i32, y: ((t[0][1] + 1.0) * 400 as f32) as i32 }, &Color {r: 255, g: 255, b: 255, a: 255 }, &mut canvas);

       /* draw_triangle(Vec2i {x: ((t[0][0] + 1.0) * 400 as f32) as i32, y: ((t[0][1] + 1.0) * 400 as f32) as i32},
                      Vec2i {x: ((t[1][0] + 1.0) * 400 as f32) as i32, y: ((t[1][1] + 1.0) * 400 as f32) as i32 },
                      Vec2i {x: ((t[2][0] + 1.0) * 400 as f32) as i32, y: ((t[2][1] + 1.0) * 400 as f32) as i32 },
                      &Color {r: 255, g: 255, b: 255, a: 255 }, &mut canvas);*/
    }

    while window.pump() {
        window.update();
    }
    Ok(())
}
