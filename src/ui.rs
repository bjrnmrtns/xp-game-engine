#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub size_px: u32,
    pub color: [u8; 4],
}

impl Text {
    pub fn new(text: &str) -> Text {
        Self {
            text: text.to_string(),
            size_px: 10,
            color: [255, 0, 0, 255],
        }
    }
    pub fn with_size_px(&self, size_px: u32) -> Text {
        let mut ret = self.clone();
        ret.size_px = size_px;
        ret
    }

    pub fn with_color(&self, color: [u8; 4]) -> Text {
        let mut ret = self.clone();
        ret.color = color;
        ret
    }
}

pub trait Widget {
    fn top_left(&self) -> Position;
    fn size(&self) -> Size;
    fn color(&self) -> [u8; 4];
}

pub struct Label {
    top_left: Position,
    size: Size,
    pub text: Text,
    color: [u8; 4],
}

impl Widget for Label {
    fn top_left(&self) -> Position {
        self.top_left
    }
    fn size(&self) -> Size {
        self.size
    }
    fn color(&self) -> [u8; 4] { self.color }
}

impl Label {
    pub fn new(top_left: Position, size: Size, text: Text) -> Label {
        Self { top_left, size, text, color: [255, 0, 0, 255] }
    }
}

pub fn create(width: u32, height: u32) -> Vec<Label> {
    let mut ui = Vec::new();
    ui.push(Label::new(Position { x: 0.0, y: height as f32 }, Size { width: 100.0, height: 100.0 }, Text::new("fps").with_size_px(48).with_color([0, 255, 0, 255])));
    ui.push(Label::new(Position { x: width as f32 - 300.0, y: height as f32 }, Size { width: 300.0, height: 40.0 }, Text::new("camera").with_size_px(32).with_color([0, 0, 255, 255])));
    ui
}
