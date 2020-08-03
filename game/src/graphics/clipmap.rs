use crate::graphics::{Drawable, texture};
use nalgebra_glm::{Mat4, identity, Vec3, vec3};
use wgpu::{Device, BindingResource, BindGroupLayoutEntry, TextureViewDimension};
use crate::graphics::error::GraphicsError;

type Result<T> = std::result::Result<T, GraphicsError>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub p: [f32; 2],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float2,
                },
            ]
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Instance {
    pub(crate) model: Mat4,
}

unsafe impl bytemuck::Pod for Instance {}
unsafe impl bytemuck::Zeroable for Instance {}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniforms {
    pub projection: Mat4,
    pub view: Mat4,
    pub camera_position: Vec3,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

pub struct Renderer {
    pub drawables: Vec<Drawable>,
    pub uniform_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
    pub texture: texture::Texture,
}

impl Renderer {
    pub async fn new(device: &Device, sc_descriptor: &wgpu::SwapChainDescriptor, queue: &wgpu::Queue) -> Result<Self> {
        // from here 3D renderpipeline creation
        let vs_spirv = glsl_to_spirv::compile(include_str!("../shader_clipmap.vert"), glsl_to_spirv::ShaderType::Vertex)?;
        let fs_spirv = glsl_to_spirv::compile(include_str!("../shader_clipmap.frag"), glsl_to_spirv::ShaderType::Fragment)?;
        let vs_data = wgpu::read_spirv(vs_spirv)?;
        let fs_data = wgpu::read_spirv(fs_spirv)?;
        let vs_module = device.create_shader_module(&vs_data);
        let fs_module = device.create_shader_module(&fs_data);

        let uniforms = Uniforms { projection: identity(), view: identity(), camera_position: vec3(0.0, 0.0, 0.0) };

        let uniform_buffer = device.create_buffer_with_data(bytemuck::cast_slice(&[uniforms]),
                                                            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST);

        let instances = [Instance { model: identity(), }];
        let instance_buffer = device.create_buffer_with_data(bytemuck::cast_slice(&instances),
                                                             wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::StorageBuffer {
                        dynamic: false,
                        readonly: false,
                    },
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        component_type: wgpu::TextureComponentType::Float,
                        dimension: TextureViewDimension::D2,
                    },
                },
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                },
            ],
            label: None,
        });

        let (texture, encoder) = texture::Texture::create_clipmap_texture(&device, 16);
        queue.submit(&[encoder.finish()]);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buffer,
                        // FYI: you can share a single buffer between bindings.
                        range: 0..std::mem::size_of_val(&uniforms) as wgpu::BufferAddress,
                    }
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &instance_buffer,
                        range: 0..std::mem::size_of_val(&instances) as wgpu::BufferAddress,
                    }
                },
                wgpu::Binding {
                    binding: 2,
                    resource: BindingResource::TextureView(&texture.view),
                },
                wgpu::Binding {
                    binding: 3,
                    resource: BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: None,
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &render_pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main"
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            color_states: &[
                wgpu::ColorStateDescriptor {
                    format: sc_descriptor.format,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }
            ],
            primitive_topology: wgpu::PrimitiveTopology::LineList,
            depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
                stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
                stencil_read_mask: 0,
                stencil_write_mask: 0,
            }),
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint32,
                vertex_buffers: &[Vertex::desc()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Ok(Self {
            drawables: Vec::new(),
            uniform_buffer,
            instance_buffer,
            bind_group: bind_group,
            render_pipeline,
            texture,
        })
    }
}

pub fn create_clipmap() -> (Vec<Vertex>, Vec<u32>) {
    const K: u32 = 4;
    const N: u32 = 15;
    assert_eq!(N, (2 as u32).pow(K) - 1);
    let mut vertices: Vec<Vertex> = Vec::new();
    for z in 0..N+1 {
        for x in 0..N+1 {
            vertices.push(Vertex {
                p: [x as f32, z as f32],
            })
        }
    }
    let mut indices: Vec<u32> = Vec::new();
    for z in 0..N {
        for x in 0..N {
            let i0 = x + z * (N+1);
            let i1 = i0 + 1;
            let i2 = x + (z + 1) * (N+1);
            let i3 = i2 + 1;
            indices.extend_from_slice(&[i0, i2, i2, i1, i1, i0, i1, i2, i2, i3, i3, i1]); // line_strip -> wireframe, indexbuffer for filled is remove even indices
        }
    }
    (vertices, indices)
}

fn get_update_region(from: i32, to: i32, size: i32) -> std::ops::Range<i32> {
    if from < to {
        let diff = std::cmp::min(size, to - from);
        ((to-diff)..to)
    } else {
        let diff = std::cmp::min(size, from - to);
        (to..(to+diff))
    }
}

#[test]
fn test_update() {
    assert_eq!(get_update_region(1, 10, 8), (2..10));
    assert_eq!(get_update_region(-2, 8, 12), (-2..8));
    assert_eq!(get_update_region(8, -2, 12), (-2..8));
    assert_eq!(get_update_region(8, -2, 6), (-2..4));
}
