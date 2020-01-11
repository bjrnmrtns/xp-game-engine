use software_renderer_rs::*;

fn main() {
    let width: usize = 640;
    let height: usize = 480;
    let mut canvas = Canvas::new(width, height, Color{r: 0, g:0, b: 255, a: 255});
    let window: Window = Window::new(&canvas);
    let mut i: u32 = 0;
    while window.pump() {
        canvas.set(10, 10, Color{r: 255 as u8, g: 0, b: 0, a: 255 });
        window.update();
        i = i + 1;
    }
}
