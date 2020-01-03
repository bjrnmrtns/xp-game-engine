#[repr(C)]
pub struct Color
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

extern "C" {
    pub fn windowing_create(buffer: *const Color, size: libc::size_t) -> *const libc::c_void;
    pub fn windowing_destroy(cookie: *const libc::c_void);
    pub fn windowing_pump(cookie: *const libc::c_void) -> bool;
    pub fn windowing_update(cookie: *const libc::c_void);
}
