use software_renderer_rs::*;

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
    for x in x0 as i32..x1 as i32 +1 {
        let t: f64 = (x as f64 - x0) / (x1 - x0);
        let y= y0 * (1.0 - t) + y1 * t;
        if steep {
            canvas.set(y as usize, x as usize, color);
        } else {
            canvas.set(x as usize, y as usize, color);
        }
    }
}

fn main() {
    let width: usize = 801;
    let height: usize = 801;
    let mut canvas = Canvas::new(width, height, Color{r: 0, g:0, b: 0, a: 255});
    let window: Window = Window::new(&canvas);
    draw_line(Vec2i {x: 130, y: 200}, Vec2i {x: 800, y: 400}, &Color {r: 255, g: 255, b: 255, a: 255 }, &mut canvas);
    draw_line(Vec2i {x: 200, y: 130}, Vec2i {x: 400, y: 800}, &Color {r: 255, g: 0, b: 0, a: 255 }, &mut canvas);
    draw_line(Vec2i {x: 800, y: 400}, Vec2i {x: 130, y: 200}, &Color {r: 255, g: 0, b: 0, a: 255 }, &mut canvas);

    while window.pump() {
        window.update();
    }
}
