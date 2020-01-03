use software_renderer_rs::*;

fn main() {
    unsafe {
        let window_size = 640 * 480;
        let mut fb: Vec<Color> = Vec::with_capacity(window_size);
        for _i in 0..window_size {
            fb.push(Color { r: 255, g: 0, b: 0, a: 255});
        }

        let context = window_create(fb.as_ptr(), fb.len() as libc::size_t);

        let mut i = 0;
        while window_pump(context) {
            fb[i % window_size] = Color {r: 0, g: 0, b: 255, a: 255 };
            window_update(context);
            i += 1;
        }
        window_destroy(context);
    }
}
