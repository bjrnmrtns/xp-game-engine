use crate::renderer::{Mesh, Renderer};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug)]
pub struct VertexBuffer {
    pub vertex_buffer: wgpu::Buffer,
    pub len: u32,
}

impl VertexBuffer {
    pub fn from_mesh(renderer: &Renderer, mesh: &Mesh) -> Self {
        let vertex_buffer = renderer.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(mesh.vertices.as_slice()),
            usage: wgpu::BufferUsage::VERTEX,
        });
        Self {
            vertex_buffer,
            len: mesh.vertices.len() as u32,
        }
    }
}
