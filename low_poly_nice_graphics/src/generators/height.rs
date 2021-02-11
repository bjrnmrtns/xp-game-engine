pub trait Height {
    fn height(&self, x: f32, y: f32) -> f32;
}

pub struct Zero;

impl Height for Zero {
    fn height(&self, _x: f32, _y: f32) -> f32 {
        0.0
    }
}
