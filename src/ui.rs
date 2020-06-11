use crate::graphics;
use image::math::utils::clamp;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String,
    pub size_px: i32,
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
    pub fn with_size_px(mut self, size_px: i32) -> Text {
        self.size_px = size_px;
        self
    }

    pub fn with_color(mut self, color: [u8; 4]) -> Text {
        self.color = color;
        self
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
    pub fn new(top_left: Position, size: Size, text: Text, color: [u8; 4]) -> Label {
        Self { top_left, size, text, color, }
    }
}

pub struct Ui {
    cursor_position: Position,
    window_size: Size,
    labels: Vec<Label>,
}

impl Ui {
    pub fn new(window_size: Size) -> Self {
        Self {
            cursor_position: Position { x: (window_size.width - 1) / 2, y: (window_size.height - 1) / 2 },
            window_size,
            labels: Ui::create_labels(window_size),
        }
    }

    fn create_labels(window_size: Size) -> Vec<Label> {
        let mut labels = Vec::new();
        labels.push(Label::new(Position { x: 0, y: window_size.height - 1 }, Size { width: 100, height: 100 }, Text::new("fps").with_size_px(48).with_color([0, 255, 0, 255]), [0, 255, 0, 255]));
        labels.push(Label::new(Position { x: window_size.width  - 1 - 300, y: window_size.height - 1 }, Size { width: 300, height: 40 }, Text::new("camera").with_size_px(32).with_color([0, 0, 255, 255]), [0, 255, 0, 255]));
        labels
    }

    pub fn update_window_size(&mut self, window_size: Size) {
        self.window_size = window_size;
        self.cursor_position.x = clamp(self.cursor_position.x, 0, window_size.width - 1);
        self.cursor_position.y = clamp(self.cursor_position.y, 0, window_size.height - 1);
        self.labels = Ui::create_labels(window_size);
    }

    pub fn create_mesh(&self) -> graphics::Mesh::<graphics::UIVertex> {
        let mut mesh = graphics::Mesh::<graphics::UIVertex> { vertices: Vec::new(), indices: Vec::new() };
        for label in &self.labels {
            let top_left = graphics::UIVertex {
                position: [label.top_left().x as f32, label.top_left().y as f32],
                uv: [0.0, 0.0],
                color: label.color(),
            };
            let bottom_left = graphics::UIVertex {
                position: [label.top_left().x as f32, (label.top_left().y - label.size().height) as f32],
                uv: [0.0, 0.0],
                color: label.color(),
            };
            let top_right = graphics::UIVertex {
                position: [(label.top_left().x + label.size().width) as f32, label.top_left().y as f32],
                uv: [0.0, 0.0],
                color: label.color(),
            };
            let bottom_right = graphics::UIVertex {
                position: [(label.top_left().x + label.size().width) as f32, (label.top_left().y - label.size().height) as f32],
                uv: [0.0, 0.0],
                color: label.color(),
            };
            let offset = mesh.vertices.len() as u32;
            mesh.indices.extend_from_slice(&[offset + 0, offset + 1, offset + 2, offset + 2, offset + 1, offset + 3]);
            mesh.vertices.extend_from_slice(&[top_left, bottom_left, top_right, bottom_right]);
        }
        let cursor_top_left = graphics::UIVertex {
            position: [self.cursor_position.x as f32, self.cursor_position.y as f32],
            uv: [0.0, 0.0],
            color: [128, 128, 128, 255],
        };
        let cursor_bottom_left = graphics::UIVertex {
            position: [self.cursor_position.x as f32, self.cursor_position.y as f32 - 50.0],
            uv: [0.0, 0.0],
            color: [128, 128, 128, 255],
        };
        let cursor_bottom_right = graphics::UIVertex {
            position: [self.cursor_position.x as f32 + 50.0, self.cursor_position.y as f32 - 50.0],
            uv: [0.0, 0.0],
            color: [128, 128, 128, 255],
        };
        let offset = mesh.vertices.len() as u32;
        mesh.indices.extend_from_slice(&[offset + 0, offset + 1, offset + 2]);
        mesh.vertices.extend_from_slice(&[cursor_top_left, cursor_bottom_left, cursor_bottom_right]);

        mesh
    }
}

