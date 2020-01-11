use software_renderer_rs::*;

fn main() {
    let window_size = 640 * 480;
    let mut fb: Vec<Color> = Vec::with_capacity(window_size);
    for _i in 0..window_size {
        fb.push(Color { r: 255, g: 0, b: 0, a: 255});
    }

    let window: Window = Window::new(fb.as_slice());
    let mut i = 0;
    while window.pump() {
        fb[i % window_size] = Color {r: 0, g: 0, b: 255, a: 255 };
        window.update();
        i += 1;
    }
}

