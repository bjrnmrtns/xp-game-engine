use crate::graphics::{Mesh, texture, Drawable};
use wgpu::{*};
use nalgebra_glm::{Mat4, identity};
use crate::graphics::error::GraphicsError;

type Result<T> = std::result::Result<T, GraphicsError>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniforms {
    pub projection: Mat4,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
    pub color: [u8; 4],
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
                wgpu::VertexAttributeDescriptor {
                    offset: 8,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float2,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: 16,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Uint,
                },
            ]
        }
    }
}

pub struct Renderer {
    pub drawable: Drawable,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
    pub render_pipeline: wgpu::RenderPipeline,
    pub texture_bind_group: wgpu::BindGroup,
}

impl Renderer {
    pub async fn new(device: &Device, sc_descriptor: &wgpu::SwapChainDescriptor, queue: &wgpu::Queue, ui_mesh: Mesh<Vertex>) -> Result<Self> {
        let vs_ui_spirv = glsl_to_spirv::compile(include_str!("../shader_ui.vert"), glsl_to_spirv::ShaderType::Vertex)?;
        let fs_ui_spirv = glsl_to_spirv::compile(include_str!("../shader_ui.frag"), glsl_to_spirv::ShaderType::Fragment)?;
        let vs_ui_data = wgpu::read_spirv(vs_ui_spirv)?;
        let fs_ui_data = wgpu::read_spirv(fs_ui_spirv)?;
        let ui_vs_module = device.create_shader_module(&vs_ui_data);
        let ui_fs_module = device.create_shader_module(&fs_ui_data);

        let ui_uniforms = Uniforms { projection: identity(), };

        let uniform_buffer = device.create_buffer_with_data(bytemuck::cast_slice(&[ui_uniforms]),
                                                            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST);

        let ui_uniform_layout= device.create_bind_group_layout(&BindGroupLayoutDescriptor{
            label: None,
            bindings: &[BindGroupLayoutEntry{
                binding: 0,
                visibility: ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false},
            }]
        });

        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor{
            label: None,
            layout: &ui_uniform_layout,
            bindings: &[wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &uniform_buffer,
                    range: 0..std::mem::size_of_val(&ui_uniforms) as wgpu::BufferAddress,
                }
            }],
        });

        let ui_texture_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor{
            label: None,
            bindings: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        component_type: wgpu::TextureComponentType::Float,
                        dimension: TextureViewDimension::D2,
                    },
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                },
            ]
        });

        let ui_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            bind_group_layouts: &[&ui_uniform_layout, &ui_texture_layout],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
            layout: &ui_pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &ui_vs_module,
                entry_point: "main"
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &ui_fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: PrimitiveTopology::TriangleList,
            color_states: &[ColorStateDescriptor {
                format: sc_descriptor.format,
                color_blend: BlendDescriptor {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                },
                alpha_blend: BlendDescriptor {
                    src_factor: BlendFactor::OneMinusDstAlpha,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Add,
                },
                write_mask: ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: VertexStateDescriptor {
                index_format: IndexFormat::Uint32,
                vertex_buffers: &[Vertex::desc()],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });
        let (ui_texture, encoder) = texture::Texture::create_ui_texture(&device);
        queue.submit(&[encoder.finish()]);
        let texture_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &ui_texture_layout,
            bindings: &[
                Binding {
                    binding: 0,
                    resource: BindingResource::TextureView(&ui_texture.view),
                },
                Binding {
                    binding: 1,
                    resource: BindingResource::Sampler(&ui_texture.sampler),
                },
            ],
        });
        Ok(Self {
            drawable: Drawable { vertex_buffer: device.create_buffer_with_data(bytemuck::cast_slice(&ui_mesh.vertices), wgpu::BufferUsage::VERTEX),
                index_buffer: device.create_buffer_with_data(bytemuck::cast_slice(&ui_mesh.indices), wgpu::BufferUsage::INDEX),
                index_buffer_len: ui_mesh.indices.len() as u32 },
            texture_bind_group,
            render_pipeline,
            uniform_bind_group,
            uniform_buffer
        })
    }
}
