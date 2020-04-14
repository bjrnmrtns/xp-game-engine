#[repr(C)]
#[derive(Copy, Clone)]
pub struct Color
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
    pub zbuffer: Vec<f32>,
}

impl Canvas
{
    pub fn new(width: usize, height: usize, color: &Color) -> Canvas {
        Canvas {
            width,
            height,
            buffer: vec![*color; width * height],
            zbuffer: vec![std::f32::MIN; width * height]
        }
    }
    pub fn clear(&mut self, color: &Color) {
        for elem in self.buffer.iter_mut() { *elem = *color; }
    }
    pub fn set_with_depth(&mut self, x: usize, y: usize, depth: f32, color: &Color) {
        if x < self.width && y < self.height {
            let index = x + (self.height - 1 - y) * self.width;
            if depth < self.zbuffer[index]  {
                self.zbuffer[index] = depth;
                self.buffer[index] = *color;
            }
        }
    }
    pub fn set(&mut self, x: usize, y: usize, color: &Color) {
        if x < self.width && y < self.height {
            self.buffer[x + (self.height - 1 - y) * self.width] = *color;
        }
    }
    pub fn clear_zbuffer(&mut self) {
        self.zbuffer = vec![std::f32::MAX; self.width * self.height];
    }
}