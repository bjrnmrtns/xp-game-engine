use crate::input::*;
use crate::canvas::{Canvas, Color};

#[repr(u32)]
#[allow(dead_code)]
enum EventTag {
    Quit,
    MouseMotion,
    KeyEvent,
    NoEvent,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct KeyEvent { key: Key, down: bool }

#[repr(C)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
struct MouseMotion { x_rel: i32, y_rel: i32 }

#[repr(C)]
#[derive(Copy, Clone)]
union EventUnion {
    quit: (),
    mouse_motion: MouseMotion,
    key_event: KeyEvent,
    no_event: (),
}

#[repr(C)]
struct InputEventData {
    tag: EventTag,
    val: EventUnion,
}

extern "C" {
    fn window_create(width: libc::size_t, height: libc::size_t, buffer: *const Color, size: libc::size_t) -> *const libc::c_void;
    fn window_destroy(handle: *const libc::c_void);
    fn window_poll_event(handle: *const libc::c_void) -> InputEventData;
    fn window_update(handle: *const libc::c_void);
}

pub struct SDLWindow
{
    handle: *const libc::c_void
}

impl SDLWindow
{
    pub fn new(canvas: &Canvas) -> Box<dyn Window> {
        unsafe {
            Box::new(SDLWindow {
                handle: window_create(canvas.width as libc::size_t, canvas.height as libc::size_t, canvas.buffer.as_ptr(), canvas.buffer.len() as libc::size_t)
            })
        }
    }
}

impl Window for SDLWindow {
    fn update(&self) {
        unsafe {
            window_update(self.handle);
        }
    }

    fn poll_input(&self) -> Option<Event> {
        unsafe {
            match window_poll_event(self.handle) {
                InputEventData { tag: EventTag::Quit, val: _ } => Some(Event::Quit),
                InputEventData { tag: EventTag::MouseMotion, val: EventUnion { mouse_motion: motion} } => Some(Event::MouseMotion { x_rel: motion.x_rel, y_rel: motion.y_rel }),
                InputEventData { tag: EventTag::KeyEvent, val: EventUnion { key_event: keyinfo} } => Some(Event::KeyEvent { key: keyinfo.key, down: keyinfo.down }),
                InputEventData { tag: EventTag::NoEvent, val: _ } => None,
            }
        }
    }
}

impl Drop for SDLWindow {
    fn drop(&mut self) {
        unsafe {
            window_destroy(self.handle);
        }
    }
}

