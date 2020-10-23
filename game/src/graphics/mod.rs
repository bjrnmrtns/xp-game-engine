use crate::graphics::error::GraphicsError;
use nalgebra_glm::{Mat4, Vec3};
use std::collections::{HashMap, HashSet};
use wgpu::util::DeviceExt;
use winit::window::Window;

pub mod clipmap;
pub mod error;
pub mod mesh;
pub mod texture;
pub mod ui;

type Result<T> = std::result::Result<T, GraphicsError>;

pub struct DrawDescription {
    name: String,
    vbi: usize,
    vb_len: usize,
    entity_ids: HashSet<u32>,
}

impl DrawDescription {
    pub fn add_entity_id(&mut self, id: u32) {
        self.entity_ids.insert(id);
    }
}

pub struct Drawables {
    buffers: Vec<wgpu::Buffer>,
    draw_descriptions: Vec<DrawDescription>,
}

impl Drawables {
    pub fn new() -> Self {
        Self {
            buffers: vec![],
            draw_descriptions: vec![],
        }
    }
    pub fn add_drawable(&mut self, name: String, vertex_buffer: wgpu::Buffer, vb_len: usize) {
        self.buffers.push(vertex_buffer);
        self.draw_descriptions.push(DrawDescription {
            name,
            vbi: self.buffers.len() - 1,
            vb_len,
            entity_ids: HashSet::new(),
        })
    }
    pub fn add_entity(&mut self, id: u32, name: &String) {
        for draw_description in &mut self.draw_descriptions {
            if &draw_description.name == name {
                draw_description.entity_ids.insert(id);
            }
        }
    }
}

pub struct Mesh<T> {
    pub vertices: Vec<T>,
    pub indices: Vec<u32>,
}

impl<T> Mesh<T> {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}

pub struct Buffer {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_buffer_len: u32,
}

pub fn create_buffer_from<
    V: bytemuck::Zeroable + bytemuck::Pod,
    I: bytemuck::Zeroable + bytemuck::Pod,
>(
    device: &wgpu::Device,
    verts_and_ind: (&[V], &[I]),
) -> Buffer {
    Buffer {
        vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(verts_and_ind.0),
            usage: wgpu::BufferUsage::VERTEX,
        }),
        index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(verts_and_ind.1),
            usage: wgpu::BufferUsage::INDEX,
        }),
        index_buffer_len: verts_and_ind.1.len() as u32,
    }
}

pub struct Graphics {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_descriptor: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub depth_texture: texture::Texture,
    window_size: (u32, u32),
    pub mesh_renderer: mesh::Renderer,
    pub clipmap_renderer: clipmap::Renderer,
    pub ui_renderer: ui::Renderer,
}

impl Graphics {
    pub fn build_glyph_brush(
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
    ) -> wgpu_glyph::GlyphBrush<()> {
        let font = wgpu_glyph::ab_glyph::FontArc::try_from_slice(include_bytes!(
            "../JetBrainsMono-Regular.ttf"
        ))
        .expect("Can not load font");
        let glyph_brush =
            wgpu_glyph::GlyphBrushBuilder::using_font(font).build(&device, texture_format);
        glyph_brush
    }

    pub async fn new(window: &Window) -> Result<Self> {
        // from here device creation and surface swapchain
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await;
        let adapter = match adapter {
            Some(adapter) => adapter,
            None => {
                return Err(GraphicsError::RequestAdapter);
            }
        };
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: Default::default(),
                    limits: Default::default(),
                    shader_validation: false,
                },
                None,
            )
            .await
            .unwrap();

        let sc_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_descriptor);
        let depth_texture = texture::Texture::create_depth_texture(&device, &sc_descriptor);
        let ui_renderer = ui::Renderer::new(&device, &sc_descriptor, &queue).await?;
        let mesh_renderer = mesh::Renderer::new(&device, &sc_descriptor, &queue).await?;
        let clipmap_renderer = clipmap::Renderer::new(&device, &sc_descriptor, &queue).await?;

        Ok(Self {
            surface,
            device,
            queue,
            sc_descriptor,
            swap_chain,
            depth_texture,
            window_size: (window.inner_size().width, window.inner_size().height),
            mesh_renderer,
            clipmap_renderer,
            ui_renderer,
        })
    }

    pub async fn resize(&mut self, width: u32, height: u32) {
        self.window_size = (width, height);
        self.sc_descriptor.width = width;
        self.sc_descriptor.height = height;
        self.depth_texture =
            texture::Texture::create_depth_texture(&self.device, &self.sc_descriptor);
        self.swap_chain = self
            .device
            .create_swap_chain(&self.surface, &self.sc_descriptor);
    }

    pub fn add_mesh_with_name<I>(&mut self, name: String, triangle_iterator: I)
    where
        I: Iterator<Item = xp_mesh::Triangle<Vec3>>,
    {
        self.mesh_renderer
            .add_mesh_with_name(&mut self.device, name, triangle_iterator);
    }

    pub fn add_entities(&mut self, mapping: &[(u32, &String)]) {
        self.mesh_renderer.add_entities(mapping);
    }

    pub fn render_loop(
        &mut self,
        entities: HashMap<u32, Mat4>,
        projection: Mat4,
        view: Mat4,
        player_view_position_for_clipmap: Vec3,
    ) {
        let target = &self
            .swap_chain
            .get_current_frame()
            .expect("failed to get next texture")
            .output
            .view;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut game_render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0),
                        store: true,
                    }),
                }),
            });
            self.mesh_renderer.render(
                &mut game_render_pass,
                &self.queue,
                projection,
                view,
                entities,
            );
            self.clipmap_renderer.render(
                &mut game_render_pass,
                &self.queue,
                &projection,
                &view,
                &player_view_position_for_clipmap,
            );
        }
        {
            let mut ui_render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            self.ui_renderer.render(&mut ui_render_pass);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
    }
}
