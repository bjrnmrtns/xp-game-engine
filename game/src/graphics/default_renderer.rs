use crate::graphics::{texture, Drawable, Mesh};
use wgpu::*;
use nalgebra_glm::{Mat4, identity};
use crate::graphics::error::GraphicsError;

type Result<T> = std::result::Result<T, GraphicsError>;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
}

impl From<&[f32; 3]> for Vertex {
    fn from(p: &[f32; 3]) -> Self {
        Self {
            position: *p,
            normal: [0.0, 1.0, 0.0],
            color: [0.0, 0.0, 0.0],
        }
    }
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
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32;3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: 2 * mem::size_of::<[f32;3]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float3,
                },
            ]
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Instance {
    pub model: Mat4,
}

unsafe impl bytemuck::Pod for Instance {}
unsafe impl bytemuck::Zeroable for Instance {}


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniforms {
    pub projection: Mat4,
    pub view: Mat4,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

pub struct Renderer {
    pub drawables: Vec<Drawable>,
    pub uniform_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
    uniforms: Uniforms,
    instances: Vec<Instance>,
}
impl Renderer {
    pub async fn new(device: &Device, sc_descriptor: &wgpu::SwapChainDescriptor, _queue: &wgpu::Queue) -> Result<Self> {
        // from here 3D renderpipeline creation
        let vs_spirv = glsl_to_spirv::compile(include_str!("../shader.vert"), glsl_to_spirv::ShaderType::Vertex)?;
        let fs_spirv = glsl_to_spirv::compile(include_str!("../shader.frag"), glsl_to_spirv::ShaderType::Fragment)?;
        let vs_data = wgpu::read_spirv(vs_spirv)?;
        let fs_data = wgpu::read_spirv(fs_spirv)?;
        let vs_module = device.create_shader_module(&vs_data);
        let fs_module = device.create_shader_module(&fs_data);

        let uniforms = Uniforms { projection: identity(), view: identity(), };

        let uniform_buffer = device.create_buffer_with_data(bytemuck::cast_slice(&[uniforms]),
                                                            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST);

        let mut instances = Vec::new();
        instances.push(Instance { model: identity(), });
        let instance_buffer = device.create_buffer_with_data(bytemuck::cast_slice(instances.as_slice()),
                                                             wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST);

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
            ],
            label: None,
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
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
            ],
            label: None,
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&uniform_bind_group_layout],
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
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
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
            uniform_bind_group,
            render_pipeline,
            uniforms,
            instances,
        })
    }

    pub fn create_drawable(&mut self, device: &wgpu::Device, mesh: &Mesh<Vertex>) -> usize {
        let vertex_buffer = device.create_buffer_with_data(bytemuck::cast_slice(&mesh.vertices), wgpu::BufferUsage::VERTEX);
        let index_buffer = device.create_buffer_with_data(bytemuck::cast_slice(&mesh.indices), wgpu::BufferUsage::INDEX);
        self.drawables.push(Drawable { vertex_buffer, index_buffer, index_buffer_len: mesh.indices.len() as u32, });
        self.drawables.len() - 1
    }

    pub fn update(&mut self, uniforms: Uniforms, instances: Vec<Instance>) {
        self.uniforms = uniforms;
        self.instances = instances;
    }

    pub fn pre_render(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        let instance_buffer = device.create_buffer_with_data(bytemuck::cast_slice(self.instances.as_slice()), wgpu::BufferUsage::COPY_SRC);
        encoder.copy_buffer_to_buffer(&instance_buffer, 0, &self.instance_buffer, 0,
                                      std::mem::size_of_val(self.instances.as_slice()) as wgpu::BufferAddress);

        let buffer = device.create_buffer_with_data(bytemuck::cast_slice(&[self.uniforms]), wgpu::BufferUsage::COPY_SRC);
        encoder.copy_buffer_to_buffer(&buffer, 0, &self.uniform_buffer, 0, std::mem::size_of_val(&self.uniforms) as u64);
    }
}