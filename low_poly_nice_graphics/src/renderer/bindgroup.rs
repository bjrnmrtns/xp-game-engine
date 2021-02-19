use crate::{
    registry::Registry,
    renderer::{
        light::{MAX_NR_OF_DIRECTIONAL_LIGHTS, MAX_NR_OF_POINT_LIGHTS, MAX_NR_OF_SPOT_LIGHTS},
        Camera, DirectionalProperties, Light, PointProperties, Renderer, SpotProperties,
    },
};
use glam::Mat4;

const MAX_NR_OF_INSTANCES: usize = 100;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniform {
    pub v: Mat4,
    pub p: Mat4,
    pub world_camera_position: [f32; 4],
    pub material_specular: [f32; 4],
    pub material_shininess: f32,
    pub nr_of_directional_lights: u32,
    pub nr_of_spot_lights: u32,
    pub nr_of_point_lights: u32,
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

pub struct BindGroup {
    pub uniform: wgpu::Buffer,
    pub instances: wgpu::Buffer,
    pub directional_lights: wgpu::Buffer,
    pub spot_lights: wgpu::Buffer,
    pub point_lights: wgpu::Buffer,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
}

impl BindGroup {
    pub fn new(renderer: &Renderer) -> Self {
        let uniform = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            size: (std::mem::size_of::<Uniform>()) as u64,
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

        let instances = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST,
            size: (std::mem::size_of::<Instance>() * MAX_NR_OF_INSTANCES) as u64,
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
                        wgpu::BindGroupLayoutEntry {
                            binding: 4,
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
                    wgpu::BindGroupEntry {
                        binding: 4,
                        resource: instances.as_entire_binding(),
                    },
                ],
            });
        Self {
            uniform,
            instances,
            directional_lights,
            spot_lights,
            point_lights,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn update_instances(&self, renderer: &Renderer, transforms: &[Instance]) {
        renderer
            .queue
            .write_buffer(&self.instances, 0, bytemuck::cast_slice(transforms));
    }

    pub fn update_uniforms(
        &self,
        renderer: &Renderer,
        lights: &Registry<Light>,
        camera: &dyn Camera,
    ) {
        let mut directional_lights = Vec::new();
        let mut spot_lights = Vec::new();
        let mut point_lights = Vec::new();
        for (_, light) in &lights.registry {
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

        let uniform = Uniform {
            v: camera.get_view(),
            p: camera.get_projection(),
            world_camera_position: [
                camera.get_position().x,
                camera.get_position().y,
                camera.get_position().z,
                1.0,
            ],
            material_specular: [0.5, 0.5, 0.5, 1.0],
            material_shininess: 16.0,
            nr_of_directional_lights: directional_lights.len() as u32,
            nr_of_spot_lights: spot_lights.len() as u32,
            nr_of_point_lights: point_lights.len() as u32,
        };
        renderer
            .queue
            .write_buffer(&self.uniform, 0, bytemuck::cast_slice(&[uniform]));

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
