use crate::renderer::{
    light::{MAX_NR_OF_POINT_LIGHTS, MAX_NR_OF_SPOT_LIGHTS},
    Camera, Renderer,
};
use glam::Mat4;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniform {
    pub v: Mat4,
    pub p: Mat4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Instance {
    pub m: Mat4,
}

unsafe impl bytemuck::Pod for Uniform {}
unsafe impl bytemuck::Zeroable for Uniform {}

unsafe impl bytemuck::Pod for Instance {}
unsafe impl bytemuck::Zeroable for Instance {}

pub struct LightBindGroup {
    pub uniform: wgpu::Buffer,
    pub instances: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl LightBindGroup {
    pub fn new(renderer: &Renderer) -> Self {
        let uniform = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            size: (std::mem::size_of::<Uniform>()) as u64,
            mapped_at_creation: false,
        });

        let instances = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST,
            size: (std::mem::size_of::<Instance>() * MAX_NR_OF_SPOT_LIGHTS * MAX_NR_OF_POINT_LIGHTS)
                as u64,
            mapped_at_creation: false,
        });

        let bind_group_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Storage { read_only: true },
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                    ],
                    label: None,
                });

        let bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: instances.as_entire_binding(),
                    },
                ],
            });
        Self {
            uniform,
            instances,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn update_uniforms(&self, renderer: &Renderer, camera: &dyn Camera) {
        let view_projection = Uniform {
            v: camera.get_view(),
            p: camera.get_projection(),
        };
        renderer
            .queue
            .write_buffer(&self.uniform, 0, bytemuck::cast_slice(&[view_projection]));
    }

    pub fn update_instances(&self, renderer: &Renderer, transforms: &[Instance]) {
        renderer
            .queue
            .write_buffer(&self.instances, 0, bytemuck::cast_slice(transforms));
    }
}
