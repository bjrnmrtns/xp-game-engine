#[repr(C)]
pub struct windowing_handle_t {
}

extern "C" {
    fn windowing_create() -> *const libc::c_void;
    fn windowing_destroy(cookie: *const libc::c_void);
    fn windowing_pump(cookie: *const libc::c_void) -> bool;
}

pub fn windowing_rs() {
    unsafe { 
        let x = windowing_create();
        while windowing_pump(x) {}
        windowing_destroy(x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn windowing_test() {
        windowing_rs()
    }
}
