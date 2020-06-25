use std::collections::hash_map::ValuesMut;
use crate::Widget;
use crate::Widget::LabelW;

#[derive(Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
pub struct Layout {
    pub position: Position,
    pub size: Size,
}

pub const DEFAULT_LAYOUT: Layout = Layout { position: Position { x: 0.0, y: 0.0 }, size: Size { width: 0.0, height: 0.0 } };

pub fn layout_basic<I>(widgets: ValuesMut<'_, I, Widget>, window_size: (f32, f32)) {
    let width: f32 = 300.0;
    let height: f32 = 34.0;
    let mut top_left_pos = Position { x: 0.0, y: window_size.1 };
    for widget in widgets {
        match widget {
            LabelW(Layout { position, size }, _) => {
                *position = top_left_pos.clone();
                *size = Size { width, height,}
            },
        }
        top_left_pos.x += width;
    }
}
