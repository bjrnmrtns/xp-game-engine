use glam::Vec2;

pub enum MouseScrollUnit {
    Line,
    Pixel,
}
pub struct MouseWheelDelta {
    pub unit: MouseScrollUnit,
    pub x: f32,
    pub y: f32,
}

pub struct MouseMotion {
    pub delta: Vec2,
}
