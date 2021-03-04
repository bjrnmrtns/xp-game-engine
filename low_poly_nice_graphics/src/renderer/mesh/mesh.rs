use crate::renderer::{Renderer, Shape};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug)]
pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub len: u32,
}

impl Mesh {
    pub fn from_shape(renderer: &Renderer, shape: Shape) -> Self {
        let vertex_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(shape.vertices.as_slice()),
                usage: wgpu::BufferUsage::VERTEX,
            });
        Self {
            vertex_buffer,
            len: shape.vertices.len() as u32,
        }
    }
}
