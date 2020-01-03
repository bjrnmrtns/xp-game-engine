use software_renderer_rs::*;

fn main() {
    unsafe {
        let context = windowing_create();
        while windowing_pump(context) {
            windowing_update(context)
        }
        windowing_destroy(context);
    }
}
