extern "C" {
    pub fn windowing_create() -> *const libc::c_void;
    pub fn windowing_destroy(cookie: *const libc::c_void);
    pub fn windowing_pump(cookie: *const libc::c_void) -> bool;
    pub fn windowing_update(cookie: *const libc::c_void);
}
