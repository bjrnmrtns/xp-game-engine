use crate::graphics;
use image::math::utils::clamp;
use std::fmt::Display;
use std::hash::Hash;
use std::collections::HashMap;
use std::ops::Index;

mod widgets;
mod layout;
mod label;
mod text;

pub use self::{
    widgets::*,
    label::*,
    text::*,
    layout::*,
};

pub struct UI<I: WidgetId = u32> {
    cursor_position: (f32, f32),
    window_size: (f32, f32),
    label_widgets: Widgets<Label, I>,
}

impl<I> UI<I> where I: WidgetId, {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            cursor_position: (width / 2.0, height / 2.0),
            window_size: (width, height),
            label_widgets: Widgets::new(),
        }
    }

    pub fn add_label(&mut self, label: Label) -> I {
        self.label_widgets.add(label)
    }

    pub fn update_window_size(&mut self, width: f32, height: f32) {
        self.window_size = (width, height);
    }

    pub fn update_cursor_position(&mut self, x: f32, y: f32) {
        self.cursor_position =  (x, self.window_size.1 - y);
    }

    pub fn click(&self) {
        println!("{} {}", self.cursor_position.0, self.cursor_position.1);
    }

    pub fn create_mesh(&self) -> graphics::Mesh::<graphics::UIVertex> {
        let mut mesh = graphics::Mesh::<graphics::UIVertex> { vertices: Vec::new(), indices: Vec::new() };
        let width: f32 = 300.0;
        let height: f32 = 300.0;
        let mut top_left_pos = (0.0, self.window_size.1);
        for label in self.label_widgets.widgets() {
            let top_left = graphics::UIVertex {
                position: [top_left_pos.0, top_left_pos.1],
                uv: [0.0, 0.0],
                color: label.color,
            };
            let bottom_left = graphics::UIVertex {
                position: [top_left_pos.0, top_left_pos.1 - height],
                uv: [0.0, 0.0],
                color: label.color,
            };
            let top_right = graphics::UIVertex {
                position: [top_left_pos.0 + width, top_left_pos.1],
                uv: [0.0, 0.0],
                color: label.color,
            };
            let bottom_right = graphics::UIVertex {
                position: [top_left_pos.0 + width, top_left_pos.1 - height],
                uv: [0.0, 0.0],
                color: label.color,
            };
            top_left_pos.0 += width;
            let offset = mesh.vertices.len() as u32;
            mesh.indices.extend_from_slice(&[offset + 0, offset + 1, offset + 2, offset + 2, offset + 1, offset + 3]);
            mesh.vertices.extend_from_slice(&[top_left, bottom_left, top_right, bottom_right]);
        }
        mesh
    }
}

