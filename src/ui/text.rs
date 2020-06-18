const DEFAULT_FONT_SIZE: f32 = 32.0;
const DEFAULT_COLOR: [u8; 4] = [255, 0, 0, 255];

pub struct Text
{
    pub text: String,
    pub font_size: f32,
    pub color: [u8; 4],
}

impl Text {
    pub fn build(text: &str) -> Self {
        Self {
            text: text.to_string(),
            font_size: DEFAULT_FONT_SIZE,
            color: DEFAULT_COLOR,
        }
    }
}