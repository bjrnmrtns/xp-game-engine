use crate::ui::{Size, Text, Widget};

pub struct Label {
    pub(crate) size: Size,
    pub text: Text,
    color: [u8; 4],
}

impl Label {
    pub fn new(size: Size, text: Text, color: [u8; 4]) -> Label {
        Self { size, text, color, }
    }
    pub fn size(&self) -> Size {
        self.size
    }
    pub fn color(&self) -> [u8; 4] { self.color }
}

impl Widget for Label {}

