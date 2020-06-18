use crate::ui::{Text, Widget};

const DEFAULT_BUTTON_COLOR: [u8; 4] = [0, 255, 0, 255];

pub struct Button {
    pub text: Text,
    pub color: [u8; 4],
}

impl Button {
    pub fn build(text: &str) -> Self {
        Self {
            text: Text::build(text),
            color: DEFAULT_BUTTON_COLOR,
        }
    }
    pub fn with_color(mut self, color: [u8; 4]) -> Self {
        self.color = color;
        self
    }
}
