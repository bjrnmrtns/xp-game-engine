use crate::graphics::error::GraphicsError;
use nalgebra_glm::Mat4;
use std::collections::{HashMap, HashSet};
use wgpu::util::DeviceExt;
use winit::window::Window;

pub mod clipmap;
pub mod error;
pub mod mesh;
pub mod mesh_debug;
pub mod texture;
pub mod ui;

type Result<T> = std::result::Result<T, GraphicsError>;

pub struct DrawDescription {
    name: String,
    index_buffer_index: usize,
    vertex_buffer_index: usize,
    index_buffer_len: usize,
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
    pub fn add_drawable(
        &mut self,
        name: String,
        vertex_buffer: wgpu::Buffer,
        index_buffer: wgpu::Buffer,
        index_buffer_len: usize,
    ) {
        self.buffers.push(vertex_buffer);
        self.buffers.push(index_buffer);
        self.draw_descriptions.push(DrawDescription {
            name,
            vertex_buffer_index: self.buffers.len() - 2,
            index_buffer_index: self.buffers.len() - 1,
            index_buffer_len,
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

pub struct Renderers {
    pub ui: ui::Renderer,
    pub default: mesh::Renderer,
    pub clipmap: clipmap::Renderer,
    pub debug: mesh_debug::Renderer,
}

pub trait Renderer {
    fn render<'a, 'b>(&'a self, render_pass: &'b mut wgpu::RenderPass<'a>)
    where
        'a: 'b;
}

impl Renderers {
    pub async fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        swapchain_descriptor: &wgpu::SwapChainDescriptor,
    ) -> Result<Self> {
        Ok(Self {
            ui: ui::Renderer::new(&device, &swapchain_descriptor, &queue).await?,
            default: mesh::Renderer::new(&device, &swapchain_descriptor, &queue).await?,
            clipmap: clipmap::Renderer::new(&device, &swapchain_descriptor, &queue).await?,
            debug: mesh_debug::Renderer::new(&device, &swapchain_descriptor, &queue).await?,
        })
    }
}

pub struct Graphics {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_descriptor: wgpu::SwapChainDescriptor,
    pub swap_chain: wgpu::SwapChain,
    pub depth_texture: texture::Texture,
    window_size: winit::dpi::PhysicalSize<u32>,
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

        Ok(Self {
            surface,
            device,
            queue,
            sc_descriptor,
            swap_chain,
            depth_texture,
            window_size: window.inner_size(),
        })
    }

    pub async fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.window_size = size;
        self.sc_descriptor.width = size.width;
        self.sc_descriptor.height = size.height;
        self.depth_texture =
            texture::Texture::create_depth_texture(&self.device, &self.sc_descriptor);
        self.swap_chain = self
            .device
            .create_swap_chain(&self.surface, &self.sc_descriptor);
    }
}

pub fn render_loop(
    renderables: &Renderers,
    entities: HashMap<u32, Mat4>,
    projection: Mat4,
    view: Mat4,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    target: &wgpu::TextureView,
    depth_attachment: &wgpu::TextureView,
) {
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
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
                attachment: &depth_attachment,
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
        renderables
            .default
            .render(&mut game_render_pass, queue, projection, view, entities);
        renderables.clipmap.render(&mut game_render_pass);
        renderables.debug.render(&mut game_render_pass);
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
        renderables.ui.render(&mut ui_render_pass);
    }
    queue.submit(std::iter::once(encoder.finish()));
}
