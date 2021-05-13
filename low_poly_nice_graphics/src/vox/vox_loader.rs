use crate::registry::{Handle, Registry};
use std::collections::HashMap;
pub struct Vox {
    data: Vec<Option<u8>>,
    palette: HashMap<u8, [f32; 3]>,
    pub x_size: usize,
    pub y_size: usize,
    pub z_size: usize,
    pub touched: bool,
}

impl Vox {
    pub fn new(x_size: usize, y_size: usize, z_size: usize) -> Self {
        Self {
            data: vec![None; z_size * y_size * x_size],
            palette: HashMap::default(),
            x_size,
            y_size,
            z_size,
            touched: false,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, color_id: u8, color: [f32; 3]) {
        self.touched = true;
        self.data[z * self.y_size * self.x_size + y * self.x_size + x] = Some(color_id);
        self.palette.insert(color_id, color);
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Option<u8> {
        self.data[z * self.y_size * self.x_size + y * self.x_size + x]
    }

    pub fn get_color(&self, color_id: u8) -> [f32; 3] {
        self.palette[&color_id]
    }
}

pub fn load_vox(data: &dot_vox::DotVoxData, registry: &mut Registry<Vox>) -> Handle<Vox> {
    let model = &data.models[0];
    let mut vox_model = Vox::new(model.size.x as usize, model.size.z as usize, model.size.y as usize);
    for v in &model.voxels {
        let color = palette_to_color(data.palette[v.i as usize]);
        vox_model.set(v.x as usize, v.z as usize, v.y as usize, v.i, color);
    }
    registry.add(vox_model)
}

fn palette_to_color(from: u32) -> [f32; 3] {
    let (_a, b, g, r) = (from >> 24 & 0xFF, from >> 16 & 0xFF, from >> 8 & 0xFF, from & 0xFF);
    [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0]
}
