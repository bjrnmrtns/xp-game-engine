#[repr(C)]
pub struct Color
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

extern "C" {
    fn window_create(buffer: *const Color, size: libc::size_t) -> *const libc::c_void;
    fn window_destroy(handle: *const libc::c_void);
    fn window_pump(handle: *const libc::c_void) -> bool;
    fn window_update(handle: *const libc::c_void);
}

pub struct Window
{
    handle: *const libc::c_void
}

impl Window
{
    pub fn new(buffer: &[Color]) -> Window {
        unsafe {
            Window {
                handle: window_create(buffer.as_ptr(), buffer.len() as libc::size_t)
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