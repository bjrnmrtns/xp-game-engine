use crate::ui::{Text, Widget};
use crate::ui::layout::Anchor;

const DEFAULT_LABEL_COLOR: [u8; 4] = [0, 255, 0, 255];

pub struct Label {
    pub text: Text,
    pub color: [u8; 4],
}

impl Label {
    pub fn build(text: &str) -> Self {
        Self {
            text: Text::build(text),
            color: DEFAULT_LABEL_COLOR,
        }
    }
    pub fn with_color(mut self, color: [u8; 4]) -> Self {
        self.color = color;
        self
    }
}

