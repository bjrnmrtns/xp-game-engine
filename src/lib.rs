#[repr(C)]
pub struct windowing_handle_t {
}

extern "C" {
    fn windowing_create() -> libc::c_void;
    fn windowing_destroy(cookie: libc::c_void) -> libc::c_void;
}

pub fn windowing_rs() {
    unsafe { 
        windowing_create();
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
