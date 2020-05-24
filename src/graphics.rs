use winit::window::Window;
use nalgebra_glm::*;
use std::collections::HashSet;
use std::convert::TryInto;

pub type Result<T> = std::result::Result<T, GraphicsError>;

#[derive(Debug)]
pub enum GraphicsError {
    String(String),
    IOError(std::io::Error),
    RequestAdapter,
}

impl From<String> for GraphicsError {
    fn from(e: String) -> GraphicsError {
        GraphicsError::String(e)
    }
}

impl From<std::io::Error> for GraphicsError {
    fn from(e: std::io::Error) -> GraphicsError {
        GraphicsError::IOError(e)
    }
}

pub fn ensure_unique_provoking_vertices(vertices: &[[f32; 3]], indices: &[u32]) -> (Vec<[f32; 3]>, Vec<u32>) {
    let mut new_vertices= vertices.to_vec();
    let mut new_indices = indices.to_vec();
    let mut provs_used: HashSet<u32> = HashSet::new();
    for face in indices.chunks(3).enumerate() {
        // first vertex of face is a provoking vertex
        if provs_used.contains(&face.1[0]) {
            new_vertices.push(vertices[face.1[0] as usize].clone());
            new_indices[&face.0 * 3] = new_vertices.len() as u32 - 1;
        } else {
            provs_used.insert(face.1[0]);
        }
    }
    (new_vertices, new_indices)
}

pub fn enhance_provoking_vertices(vertices: &[[f32; 3]], indices: &[u32]) -> Vec<Vertex> {
    let mut mesh_vertices: Vec<Vertex> = vertices.iter().map(|v| Vertex { position: *v, normal: [0.0, 1.0, 0.0], color_id: 2 } ).collect();
    for face in indices.chunks(3) {
        let edge_0: Vec3 = make_vec3(&vertices[face[2] as usize]) - make_vec3(&vertices[face[0] as usize]);
        let edge_1: Vec3 = make_vec3(&vertices[face[1] as usize]) - make_vec3(&vertices[face[0] as usize]);
        let n: Vec3 = cross(&edge_0, &edge_1).normalize();
        mesh_vertices[face[0] as usize].normal = n.as_slice().try_into().unwrap();
    }
    mesh_vertices
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float; // 1.
    pub fn create_depth_texture(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor) -> Self {
        let size = wgpu::Extent3d { // 2.
            width: sc_desc.width,
            height: sc_desc.height,
            depth: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: None,
            size,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT // 3.
                | wgpu::TextureUsage::SAMPLED
                | wgpu::TextureUsage::COPY_SRC,
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_default_view();
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor { // 4.
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::LessEqual, // 5.
        });

        Self { texture, view, sampler }
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color_id: u32, // check if this can be a byte (will index into a colormap)
}

impl From<&[f32; 3]> for Vertex {
    fn from(p: &[f32; 3]) -> Self {
        Self {
            position: *p,
            normal: [0.0, 1.0, 0.0],
            color_id: 0,
        }
    }
}


unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
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
                    format: wgpu::VertexFormat::Uint,
                },
            ]
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniforms {
    projection: Mat4,
    view: Mat4,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Instance {
    model: Mat4,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

struct Drawable {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_buffer_len: u32,
}

unsafe impl bytemuck::Pod for Instance {}
unsafe impl bytemuck::Zeroable for Instance {}

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_descriptor: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    render_pipeline: wgpu::RenderPipeline,
    drawables: Vec<Drawable>,
    uniform_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    depth_texture: Texture,
    window_size: winit::dpi::PhysicalSize<u32>,
}

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self> {
        let surface =  wgpu::Surface::create(window);
        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface) }, wgpu::BackendBit::PRIMARY).await;
        let adapter = match adapter {
            Some(adapter) => adapter,
            None => { return Err(GraphicsError::RequestAdapter); },
        };
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor
        { extensions: wgpu::Extensions { anisotropic_filtering: false, }, limits: Default::default(), }).await;
        let sc_descriptor = wgpu::SwapChainDescriptor{
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_descriptor);

        let vs_spirv = glsl_to_spirv::compile(include_str!("shader.vert"), glsl_to_spirv::ShaderType::Vertex)?;
        let fs_spirv = glsl_to_spirv::compile(include_str!("shader.frag"), glsl_to_spirv::ShaderType::Fragment)?;
        let vs_data = wgpu::read_spirv(vs_spirv)?;
        let fs_data = wgpu::read_spirv(fs_spirv)?;
        let vs_module = device.create_shader_module(&vs_data);
        let fs_module = device.create_shader_module(&fs_data);

        let uniforms = Uniforms { projection: identity(), view: identity(), };

        let uniform_buffer = device.create_buffer_with_data(bytemuck::cast_slice(&[uniforms]),
                                                            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST);

        let instances = [Instance { model: identity(), }];
        let instance_buffer = device.create_buffer_with_data(bytemuck::cast_slice(&instances),
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
                format: Texture::DEPTH_FORMAT,
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
        let depth_texture = Texture::create_depth_texture(&device, &sc_descriptor);
        Ok(Self {
            surface,
            device,
            queue,
            sc_descriptor,
            swap_chain,
            render_pipeline,
            drawables: Vec::new(),
            uniform_buffer,
            instance_buffer,
            uniform_bind_group,
            depth_texture,
            window_size: window.inner_size(),
        })
    }

    pub async fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.window_size = size;
        self.sc_descriptor.width = size.width;
        self.sc_descriptor.height = size.height;
        self.depth_texture = Texture::create_depth_texture(&self.device, &self.sc_descriptor);
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_descriptor);
    }

    pub fn create_drawable_from_mesh(&mut self, mesh: &Mesh) -> usize {
        let vertex_buffer = self.device.create_buffer_with_data(bytemuck::cast_slice(&mesh.vertices), wgpu::BufferUsage::VERTEX);
        let index_buffer = self.device.create_buffer_with_data(bytemuck::cast_slice(&mesh.indices), wgpu::BufferUsage::INDEX);
        self.drawables.push(Drawable { vertex_buffer, index_buffer, index_buffer_len: mesh.indices.len() as u32, });
        self.drawables.len() - 1
    }

    pub fn update(&mut self, model_player: Mat4, model_terrain: Mat4) {
        let instances = [Instance { model: model_player }, Instance { model: model_terrain }];
        let buffer = self.device.create_buffer_with_data(bytemuck::cast_slice(&instances), wgpu::BufferUsage::COPY_SRC);
        let mut encoder =
            self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(&buffer, 0, &self.instance_buffer, 0,
                                      std::mem::size_of_val(&instances) as wgpu::BufferAddress);
        self.queue.submit(&[encoder.finish()]);
    }

    pub async fn render(&mut self, view: Mat4) {
        let projection = perspective(self.sc_descriptor.width as f32 / self.sc_descriptor.height as f32,45.0, 0.1, 100.0);
        let uniforms = Uniforms { projection: projection, view: view, };
        let buffer = self.device.create_buffer_with_data(bytemuck::cast_slice(&[uniforms]), wgpu::BufferUsage::COPY_SRC);
        let frame = self.swap_chain.get_next_texture().expect("failed to get next texture");
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: None,
        });
        encoder.copy_buffer_to_buffer(&buffer, 0, &self.uniform_buffer, 0, std::mem::size_of_val(&uniforms) as u64);
        {
            let mut diffuse_scene_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }
                    }
                ],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_texture.view,
                    depth_load_op: wgpu::LoadOp::Clear,
                    depth_store_op: wgpu::StoreOp::Store,
                    clear_depth: 1.0,
                    stencil_load_op: wgpu::LoadOp::Clear,
                    stencil_store_op: wgpu::StoreOp::Store,
                    clear_stencil: 0,
                }),
            });
            diffuse_scene_pass.set_pipeline(&self.render_pipeline);

            diffuse_scene_pass.set_vertex_buffer(0, &self.drawables[0].vertex_buffer, 0, 0);
            diffuse_scene_pass.set_index_buffer(&self.drawables[0].index_buffer, 0, 0);
            diffuse_scene_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            diffuse_scene_pass.draw_indexed(0..self.drawables[0].index_buffer_len, 0, 0..1);

            diffuse_scene_pass.set_vertex_buffer(0, &self.drawables[1].vertex_buffer, 0, 0);
            diffuse_scene_pass.set_index_buffer(&self.drawables[1].index_buffer, 0, 0);
            diffuse_scene_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            diffuse_scene_pass.draw_indexed(0..self.drawables[1].index_buffer_len, 0, 1..2);
        }
        self.queue.submit(&[encoder.finish()]);
    }
}
