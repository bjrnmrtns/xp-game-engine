#[repr(C)]
pub struct Color
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

extern "C" {
    pub fn window_create(buffer: *const Color, size: libc::size_t) -> *const libc::c_void;
    pub fn window_destroy(cookie: *const libc::c_void);
    pub fn window_pump(cookie: *const libc::c_void) -> bool;
    pub fn window_update(cookie: *const libc::c_void);
}
