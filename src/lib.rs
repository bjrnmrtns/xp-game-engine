#[repr(C)]
#[derive(Copy)]
#[derive(Clone)]
pub struct Color
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

extern "C" {
    fn window_create(width: libc::size_t, height: libc::size_t, buffer: *const Color, size: libc::size_t) -> *const libc::c_void;
    fn window_destroy(handle: *const libc::c_void);
    fn window_pump(handle: *const libc::c_void) -> bool;
    fn window_update(handle: *const libc::c_void);
}

pub struct Window
{
    handle: *const libc::c_void
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
}

impl Canvas
{
    pub fn new(width: usize, height: usize, color: Color) -> Canvas {
        Canvas {
            width,
            height,
            buffer: vec![color; width * height]
        }
    }
    pub fn set(&mut self, x: usize, y: usize, color: &Color) {
        if x < self.width && y < self.height {
            self.buffer[x + (self.height - 1 - y) * self.width] = *color;
        }
    }
}

impl Window
{
    pub fn new(canvas: &Canvas) -> Window {
        unsafe {
            Window {
                handle: window_create(canvas.width as libc::size_t, canvas.height as libc::size_t, canvas.buffer.as_ptr(), canvas.buffer.len() as libc::size_t)
            }
        }
    }
    pub fn update(&self) {
        unsafe {
            window_update(self.handle);
        }
    }
    pub fn pump(&self) -> bool {
        unsafe {
            window_pump(self.handle)
        }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            window_destroy(self.handle);
        }
    }
}

