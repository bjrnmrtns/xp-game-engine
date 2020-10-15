use crate::entities::Entity;
use crate::graphics::error::GraphicsError;
use crate::graphics::{texture, Buffer, Drawable, Mesh};
use crate::{entities, graphics};
use nalgebra_glm::{identity, Mat4};
use std::io::Read;
use wgpu::util::DeviceExt;

type Result<T> = std::result::Result<T, GraphicsError>;

const MAX_NUMBER_OF_INSTANCES: usize = 16;

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
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: 2 * mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float3,
                },
            ],
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

pub struct NamedBuffer {
    pub name: String,
    pub buffer: Buffer,
    pub instance_range: Option<std::ops::Range<u32>>,
}

pub struct Renderable {
    pub named_buffers: Vec<NamedBuffer>,
    pub uniform_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
    uniforms: Uniforms,
}
impl Renderable {
    pub async fn new(
        device: &wgpu::Device,
        sc_descriptor: &wgpu::SwapChainDescriptor,
        _queue: &wgpu::Queue,
    ) -> Result<Self> {
        let (mut spirv_vs_bytes, mut spirv_fs_bytes) = (Vec::new(), Vec::new());
        match glsl_to_spirv::compile(
            include_str!("../shaders/shader.vert"),
            glsl_to_spirv::ShaderType::Vertex,
        ) {
            Ok(mut spirv_vs_output) => {
                spirv_vs_output.read_to_end(&mut spirv_vs_bytes).unwrap();
            }
            Err(ref e) => {
                return Err(GraphicsError::from(e.clone()));
            }
        }
        match glsl_to_spirv::compile(
            include_str!("../shaders/shader.frag"),
            glsl_to_spirv::ShaderType::Fragment,
        ) {
            Ok(mut spirv_vs_output) => {
                spirv_vs_output.read_to_end(&mut spirv_fs_bytes).unwrap();
            }
            Err(ref e) => {
                return Err(GraphicsError::from(e.clone()));
            }
        }
        let vs_module_source = wgpu::util::make_spirv(spirv_vs_bytes.as_slice());
        let fs_module_source = wgpu::util::make_spirv(spirv_fs_bytes.as_slice());
        let vs_module = device.create_shader_module(vs_module_source);
        let fs_module = device.create_shader_module(fs_module_source);

        let uniforms = Uniforms {
            projection: identity(),
            view: identity(),
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST,
            size: (std::mem::size_of::<Instance>() * MAX_NUMBER_OF_INSTANCES) as u64,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::UniformBuffer {
                            dynamic: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::VERTEX,
                        ty: wgpu::BindingType::StorageBuffer {
                            dynamic: false,
                            min_binding_size: None,
                            readonly: false,
                        },
                        count: None,
                    },
                ],
                label: None,
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(instance_buffer.slice(..)),
                },
            ],
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                clamp_depth: false,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_descriptor.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilStateDescriptor {
                    front: wgpu::StencilStateFaceDescriptor::IGNORE,
                    back: wgpu::StencilStateFaceDescriptor::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
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
            named_buffers: Vec::new(),
            uniform_buffer,
            instance_buffer,
            uniform_bind_group,
            render_pipeline,
            uniforms,
        })
    }

    pub fn get_graphics_handle(&self, name: &str) -> Option<usize> {
        for d in self.named_buffers.iter().enumerate() {
            if d.1.name == name.to_string() {
                return Some(d.0);
            }
        }
        return None;
    }

    pub fn create_drawable(
        &mut self,
        device: &wgpu::Device,
        name: String,
        mesh: &Mesh<Vertex>,
    ) -> usize {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(mesh.vertices.as_slice()),
            usage: wgpu::BufferUsage::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(mesh.indices.as_slice()),
            usage: wgpu::BufferUsage::INDEX,
        });
        self.named_buffers.push(NamedBuffer {
            name,
            buffer: Buffer {
                vertex_buffer,
                index_buffer,
                index_buffer_len: mesh.indices.len() as u32,
            },
            instance_range: None,
        });
        self.named_buffers.len() - 1
    }

    pub fn pre_render(
        &mut self,
        queue: &wgpu::Queue,
        uniforms: Uniforms,
        entities: &entities::Entities,
    ) {
        assert!(entities.len() <= MAX_NUMBER_OF_INSTANCES);
        self.uniforms = uniforms;
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
        let mut instances_total = 0;
        let mut instances = Vec::new();
        for named_buffer in self.named_buffers.iter_mut().enumerate() {
            instances.extend(entities.get_entities().iter().filter_map(|d| {
                if let Some(handle) = d.graphics_handle() {
                    if handle == named_buffer.0 {
                        return Some(Instance {
                            model: d.model_matrix(),
                        });
                    }
                }
                return None;
            }));
            if !instances.is_empty() {
                named_buffer.1.instance_range =
                    Some(instances_total as u32..(instances_total + instances.len()) as u32);
                instances_total += instances.len();
            }
        }
        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(instances.as_slice()),
        );
    }
}

impl graphics::Renderable for Renderable {
    fn render<'a, 'b>(&'a self, render_pass: &'b mut wgpu::RenderPass<'a>)
    where
        'a: 'b,
    {
        render_pass.set_pipeline(&self.render_pipeline);

        for named_buffer in &self.named_buffers {
            if let Some(instance_range) = &named_buffer.instance_range {
                render_pass.set_vertex_buffer(0, named_buffer.buffer.vertex_buffer.slice(..));
                render_pass.set_index_buffer(named_buffer.buffer.index_buffer.slice(..));
                render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
                render_pass.draw_indexed(
                    0..named_buffer.buffer.index_buffer_len,
                    0,
                    instance_range.clone(),
                );
            }
        }
    }
}
