use crate::{
    assets::Assets,
    entity::Entity,
    renderer::{
        light::{MAX_NR_OF_DIRECTIONAL_LIGHTS, MAX_NR_OF_POINT_LIGHTS, MAX_NR_OF_SPOT_LIGHTS},
        DirectionalProperties, Light, PointProperties, Renderer, SpotProperties,
    },
};
use nalgebra_glm::Mat4;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub m: Mat4,
    pub v: Mat4,
    pub p: Mat4,
    pub world_camera_position: [f32; 4],
    pub material_specular: [f32; 4],
    pub material_shininess: f32,
}

unsafe impl bytemuck::Pod for Transform {}
unsafe impl bytemuck::Zeroable for Transform {}

pub struct BindGroup {
    pub transforms: wgpu::Buffer,
    pub directional_lights: wgpu::Buffer,
    pub spot_lights: wgpu::Buffer,
    pub point_lights: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl BindGroup {
    pub fn new(renderer: &Renderer) -> Self {
        let transforms = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            size: (std::mem::size_of::<Transform>()) as u64,
            mapped_at_creation: false,
        });

        let directional_lights = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            size: (std::mem::size_of::<DirectionalProperties>() * MAX_NR_OF_DIRECTIONAL_LIGHTS)
                as u64,
            mapped_at_creation: false,
        });
        let spot_lights = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            size: (std::mem::size_of::<SpotProperties>() * MAX_NR_OF_SPOT_LIGHTS) as u64,
            mapped_at_creation: false,
        });

        let point_lights = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            size: (std::mem::size_of::<PointProperties>() * MAX_NR_OF_POINT_LIGHTS) as u64,
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
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                min_binding_size: None,
                                has_dynamic_offset: false,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
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
                        resource: transforms.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: directional_lights.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: spot_lights.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: point_lights.as_entire_binding(),
                    },
                ],
            });
        Self {
            transforms,
            directional_lights,
            spot_lights,
            point_lights,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn update_instance(
        &self,
        renderer: &Renderer,
        model: Mat4,
        projection: Mat4,
        view: Mat4,
        world_camera_position: [f32; 4],
    ) {
        let transforms = Transform {
            m: model,
            v: view,
            p: projection,
            world_camera_position,
            material_specular: [0.5, 0.5, 0.5, 1.0],
            material_shininess: 16.0,
        };
        renderer
            .queue
            .write_buffer(&self.transforms, 0, bytemuck::cast_slice(&[transforms]));
    }

    pub fn update_lights(&self, renderer: &Renderer, lights: &Assets<Light>) {
        let mut directional_lights = Vec::new();
        let mut spot_lights = Vec::new();
        let mut point_lights = Vec::new();
        for (_, light) in &lights.assets {
            match light {
                Light::Directional(properties) => {
                    directional_lights.push(*properties);
                }
                Light::Spot(properties) => {
                    spot_lights.push(*properties);
                }
                Light::Point(properties) => {
                    point_lights.push(*properties);
                }
            }
        }
        assert!(directional_lights.len() <= MAX_NR_OF_DIRECTIONAL_LIGHTS);
        assert!(spot_lights.len() <= MAX_NR_OF_SPOT_LIGHTS);
        assert!(point_lights.len() <= MAX_NR_OF_POINT_LIGHTS);
        renderer.queue.write_buffer(
            &self.directional_lights,
            0,
            bytemuck::cast_slice(directional_lights.as_slice()),
        );
        renderer.queue.write_buffer(
            &self.spot_lights,
            0,
            bytemuck::cast_slice(spot_lights.as_slice()),
        );
        renderer.queue.write_buffer(
            &self.point_lights,
            0,
            bytemuck::cast_slice(point_lights.as_slice()),
        );
    }
}
