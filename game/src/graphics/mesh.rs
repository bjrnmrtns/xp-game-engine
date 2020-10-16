use crate::graphics;
use crate::graphics::error::GraphicsError;
use crate::graphics::{texture, Drawables};
use nalgebra_glm::{identity, triangle_normal, vec3, Mat4, Vec3};
use std::collections::HashMap;
use std::io::Read;
use wgpu::util::DeviceExt;
use xp_mesh::Triangle;

type Result<T> = std::result::Result<T, GraphicsError>;

const MAX_NUMBER_OF_INSTANCES: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
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

pub struct Renderer {
    drawables: Drawables,
    uniform_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
}
impl Renderer {
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
            drawables: Drawables::new(),
            uniform_buffer,
            instance_buffer,
            uniform_bind_group,
            render_pipeline,
        })
    }

    pub fn add_mesh_with_name2<I>(
        &mut self,
        device: &wgpu::Device,
        name: String,
        triangle_iterator: I,
    ) where
        I: Iterator<Item = Triangle<Vec3>>,
    {
        let mut vs = Vec::new();
        for t in triangle_iterator {
            let normal = triangle_normal(&t.positions[0], &t.positions[1], &t.positions[2]);
            let diffuse_color = if let Some(diffuse_color) = t.diffuse_color {
                diffuse_color
            } else {
                vec3(0.5, 0.5, 0.5)
            };
            vs.extend_from_slice(&[
                Vertex {
                    position: t.positions[0].clone(),
                    normal,
                    color: diffuse_color,
                },
                Vertex {
                    position: t.positions[1].clone(),
                    normal,
                    color: diffuse_color,
                },
                Vertex {
                    position: t.positions[2].clone(),
                    normal,
                    color: diffuse_color,
                },
            ]);
        }
        let vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vs.as_slice()),
            usage: wgpu::BufferUsage::VERTEX,
        });
        self.drawables.add_drawable(name, vb, vs.len());
    }

    pub fn add_entity(&mut self, id: u32, name: &String) {
        self.drawables.add_entity(id, name);
    }

    pub fn render<'a, 'b>(
        &'a self,
        render_pass: &'b mut wgpu::RenderPass<'a>,
        queue: &wgpu::Queue,
        projection: Mat4,
        view: Mat4,
        entities: HashMap<u32, Mat4>,
    ) where
        'a: 'b,
    {
        let uniforms = graphics::mesh::Uniforms {
            projection: projection,
            view: view,
        };
        assert!(entities.len() <= MAX_NUMBER_OF_INSTANCES);
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
        let mut instances = Vec::new();
        let mut instance_ranges = Vec::new();
        let mut end_previous = 0;
        for draw_description in &self.drawables.draw_descriptions {
            instances.extend(draw_description.entity_ids.iter().filter_map(|id| {
                let model = entities.get(id);
                if let Some(model) = model {
                    return Some(Instance {
                        model: model.clone(),
                    });
                } else {
                    return None;
                }
            }));
            instance_ranges.push(end_previous as u32..(end_previous + instances.len()) as u32);
            end_previous += instances.len();
        }
        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(instances.as_slice()),
        );
        render_pass.set_pipeline(&self.render_pipeline);

        for (index, draw_description) in self.drawables.draw_descriptions.iter().enumerate() {
            render_pass
                .set_vertex_buffer(0, self.drawables.buffers[draw_description.vbi].slice(..));
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw(
                0..draw_description.vb_len as u32,
                instance_ranges[index].clone(),
            );
        }
    }
}
