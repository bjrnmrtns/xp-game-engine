pub enum Event {
    Quit,
    MouseMotion { x_rel: i32, y_rel: i32 },
    KeyEvent { key: Key, down: bool },
}

#[repr(i32)]
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum  Key {
    KeyW,
    KeyA,
    KeyS,
    KeyD,
}

pub trait Window {
    fn update(&self);
    fn poll_input(&self) -> Option<Event>;
}
