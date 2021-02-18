use crate::{
    entity::Entity,
    registry::Registry,
    renderer::{
        light::{MAX_NR_OF_DIRECTIONAL_LIGHTS, MAX_NR_OF_POINT_LIGHTS, MAX_NR_OF_SPOT_LIGHTS},
        DirectionalProperties, Light, PointProperties, Renderer, SpotProperties,
    },
};
use nalgebra_glm::Mat4;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ViewProjection {
    pub v: Mat4,
    pub p: Mat4,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub m: Mat4,
}

unsafe impl bytemuck::Pod for ViewProjection {}
unsafe impl bytemuck::Zeroable for ViewProjection {}

unsafe impl bytemuck::Pod for Transform {}
unsafe impl bytemuck::Zeroable for Transform {}

pub struct LightBindGroup {
    pub view_projection: wgpu::Buffer,
    pub transforms: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl LightBindGroup {
    pub fn new(renderer: &Renderer) -> Self {
        let view_projection = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            size: (std::mem::size_of::<ViewProjection>()) as u64,
            mapped_at_creation: false,
        });

        let transforms = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST,
            size: (std::mem::size_of::<Transform>()
                * MAX_NR_OF_SPOT_LIGHTS
                * MAX_NR_OF_POINT_LIGHTS) as u64,
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
                        resource: view_projection.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: transforms.as_entire_binding(),
                    },
                ],
            });
        Self {
            view_projection,
            transforms,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn update_view_projection(&self, renderer: &Renderer, projection: Mat4, view: Mat4) {
        let view_projection = ViewProjection {
            v: view,
            p: projection,
        };
        renderer.queue.write_buffer(
            &self.view_projection,
            0,
            bytemuck::cast_slice(&[view_projection]),
        );
    }

    pub fn update_instance(&self, renderer: &Renderer, transforms: &[Transform]) {
        renderer
            .queue
            .write_buffer(&self.transforms, 0, bytemuck::cast_slice(transforms));
    }
}
