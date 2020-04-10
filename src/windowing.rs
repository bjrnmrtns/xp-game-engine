#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[repr(u32)]
#[derive(Copy, Clone)]
enum InputEventTag {
    Quit,
    MouseMotion,
    KeyEvent,
    NotImplemented,
    NoEvent,
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum  Key {
    key_w,
    key_a,
    key_s,
    key_d,
    not_mapped,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct MouseMotionType { x_rel: i32, y_rel: i32 }

#[repr(C)]
#[derive(Copy, Clone)]
struct KeyEventType { key: Key, down: bool }

#[repr(C)]
#[derive(Copy, Clone)]
union InputEventUnion {
    Quit: (),
    MouseMotion: MouseMotionType,
    KeyEvent: KeyEventType,
    NotImplemented: (),
    NoEvent: (),
}

#[repr(C)]
struct InputEventData {
    tag: InputEventTag,
    val: InputEventUnion,
}

pub enum InputEvent {
    Quit,
    MouseMotion { x_rel: i32, y_rel: i32 },
    KeyEvent { key: Key, down: bool },
    NotImplemented,
    NoEvent,
}

extern "C" {
    fn window_create(width: libc::size_t, height: libc::size_t, buffer: *const Color, size: libc::size_t) -> *const libc::c_void;
    fn window_destroy(handle: *const libc::c_void);
    fn window_poll_event(handle: *const libc::c_void) -> InputEventData;
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
    pub zbuffer: Vec<f32>,
}

impl Canvas
{
    pub fn new(width: usize, height: usize, color: &Color) -> Canvas {
        Canvas {
            width,
            height,
            buffer: vec![*color; width * height],
            zbuffer: vec![std::f32::MIN; width * height]
        }
    }
    pub fn clear(&mut self, color: &Color) {
        for elem in self.buffer.iter_mut() { *elem = *color; }
    }
    pub fn set_with_depth(&mut self, x: usize, y: usize, depth: f32, color: &Color) {
        if x < self.width && y < self.height {
            let index = x + (self.height - 1 - y) * self.width;
            if self.zbuffer[index] < depth {
                self.zbuffer[index] = depth;
                self.buffer[index] = *color;
            }
        }
    }
    pub fn set(&mut self, x: usize, y: usize, color: &Color) {
        if x < self.width && y < self.height {
            self.buffer[x + (self.height - 1 - y) * self.width] = *color;
        }
    }
    pub fn clear_zbuffer(&mut self) {
        self.zbuffer = vec![std::f32::MIN; self.width * self.height];
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
    pub fn poll_event(&self) -> InputEvent {
        unsafe {
            match window_poll_event(self.handle) {
                InputEventData { tag: InputEventTag::Quit, val: Quit } => InputEvent::Quit,
                InputEventData { tag: InputEventTag::MouseMotion, val: InputEventUnion { MouseMotion: motion} } => InputEvent::MouseMotion { x_rel: motion.x_rel, y_rel: motion.y_rel },
                InputEventData { tag: InputEventTag::KeyEvent, val: InputEventUnion { KeyEvent: keyinfo} } => InputEvent::KeyEvent { key: keyinfo.key, down: keyinfo.down },
                InputEventData { tag: InputEventTag::NotImplemented, val: NotImplemented } => InputEvent::NotImplemented,
                InputEventData { tag: InputEventTag::NoEvent, val: NoEvent } => InputEvent::NoEvent,
            }
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

