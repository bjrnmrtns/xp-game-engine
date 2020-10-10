use crate::graphics;
use crate::graphics::error::GraphicsError;
use crate::graphics::{texture, Buffer, Graphics, Mesh};
use nalgebra_glm::{identity, Mat4};
use std::io::Read;
use wgpu::util::DeviceExt;
use wgpu::*;

type Result<T> = std::result::Result<T, GraphicsError>;

pub struct Text {
    pub pos: (f32, f32),
    pub text: String,
    pub font_size: f32,
    pub color: [u8; 4],
}

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
            ],
        }
    }
}

pub struct Renderable {
    pub drawable: Option<Buffer>,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_buffer: wgpu::Buffer,
    pub render_pipeline: wgpu::RenderPipeline,
    pub texture_bind_group: wgpu::BindGroup,
    pub glyph_brush: wgpu_glyph::GlyphBrush<()>,
    uniforms: Uniforms,
    enabled: bool,
}

impl Renderable {
    pub async fn new(
        device: &Device,
        sc_descriptor: &wgpu::SwapChainDescriptor,
        queue: &wgpu::Queue,
    ) -> Result<Self> {
        let (mut spirv_vs_bytes, mut spirv_fs_bytes) = (Vec::new(), Vec::new());
        match glsl_to_spirv::compile(
            include_str!("../shaders/shader_ui.vert"),
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
            include_str!("../shaders/shader_ui.frag"),
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
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let ui_uniform_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer {
                    dynamic: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let uniform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &ui_uniform_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    uniform_buffer.slice(0..std::mem::size_of_val(&uniforms) as u64),
                ),
            }],
        });

        let ui_texture_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        component_type: wgpu::TextureComponentType::Float,
                        dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                    count: None,
                },
            ],
        });

        let ui_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&ui_uniform_layout, &ui_texture_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&ui_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: CullMode::Back,
                clamp_depth: false,
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
        let mut command_buffers = Vec::new();
        command_buffers.push(encoder.finish());
        queue.submit(command_buffers);
        let texture_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &ui_texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&ui_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&ui_texture.sampler),
                },
            ],
        });

        let glyph_brush = Graphics::build_glyph_brush(&device, wgpu::TextureFormat::Bgra8UnormSrgb);
        Ok(Self {
            drawable: None,
            texture_bind_group,
            render_pipeline,
            uniform_bind_group,
            uniform_buffer,
            glyph_brush,
            uniforms,
            enabled: false,
        })
    }

    pub fn create_drawable(
        &mut self,
        device: &wgpu::Device,
        ui_mesh: Option<(Mesh<Vertex>, Vec<Text>)>,
    ) {
        if let Some(ui_mesh) = ui_mesh {
            self.drawable = Some(Buffer {
                vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(ui_mesh.0.vertices.as_slice()),
                    usage: wgpu::BufferUsage::VERTEX,
                }),
                index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(ui_mesh.0.indices.as_slice()),
                    usage: wgpu::BufferUsage::INDEX,
                }),
                index_buffer_len: ui_mesh.0.indices.len() as u32,
            });

            for text in &ui_mesh.1 {
                let section = wgpu_glyph::Section {
                    screen_position: text.pos,
                    text: vec![wgpu_glyph::Text::new(&text.text.as_str())
                        .with_color([1.0, 0.0, 0.0, 1.0])
                        .with_scale(wgpu_glyph::ab_glyph::PxScale {
                            x: text.font_size,
                            y: text.font_size,
                        })],
                    ..wgpu_glyph::Section::default()
                };
                self.glyph_brush.queue(section);
            }
        }
    }

    pub fn pre_render(&mut self, queue: &wgpu::Queue, uniforms: Uniforms, enabled: bool) {
        self.enabled = enabled;
        self.uniforms = uniforms;
        queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
    }
}
impl graphics::Renderable for Renderable {
    fn render<'a, 'b>(&'a self, render_pass: &'b mut RenderPass<'a>)
    where
        'a: 'b,
    {
        if self.enabled {
            if let Some(drawable) = &self.drawable {
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_vertex_buffer(0, drawable.vertex_buffer.slice(..));
                render_pass.set_index_buffer(drawable.index_buffer.slice(..));
                render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
                render_pass.set_bind_group(1, &self.texture_bind_group, &[]);
                render_pass.draw_indexed(0..drawable.index_buffer_len, 0, 0..1);
            }
            //TODO: render glyphs without mut in some way, self.glyph_brush.draw_queued(&graphics.device, &mut encoder, target, graphics.sc_descriptor.width, graphics.sc_descriptor.height,).expect("Cannot draw glyph_brush");
        }
    }
}
