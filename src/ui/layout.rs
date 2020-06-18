pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct Size {
    pub width: f32,
    pub height: f32,
}

pub struct Layout {
    pub position: Position,
    pub size: Size,
}

pub const DEFAULT_LAYOUT: Layout = Layout { position: Position { x: 0.0, y: 0.0 }, size: Size { width: 0.0, height: 0.0 } };
