use winit::window::Window;
use crate::graphics::error::GraphicsError;
use wgpu::util::DeviceExt;

pub mod default;
pub mod ui;
pub mod texture;
pub mod error;
pub mod helpers;
pub mod clipmap;

type Result<T> = std::result::Result<T, GraphicsError>;

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

pub struct Drawable {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_buffer_len: u32,
}

pub fn create_drawable_from<V: bytemuck::Zeroable + bytemuck::Pod, I: bytemuck::Zeroable + bytemuck::Pod>(device: &wgpu::Device, verts_and_ind: (&[V], &[I])) -> Drawable {
    Drawable {
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

pub struct Renderables {
    pub ui: ui::Renderable,
    pub default: default::Renderable,
    pub clipmap: clipmap::Renderable,
}

pub trait Renderable {
    fn render<'a, 'b>(&'a self, device: &wgpu::Device, queue: &wgpu::Queue, encoder: &mut wgpu::CommandEncoder, render_pass: &'b mut wgpu::RenderPass<'a>)
        where 'a: 'b;
}

impl Renderables {
    pub async fn new(device: &wgpu::Device, queue: &wgpu::Queue, swapchain_descriptor: &wgpu::SwapChainDescriptor) -> Result<Self> {
        Ok(Self {
            ui: ui::Renderable::new(&device, &swapchain_descriptor, &queue).await?,
            default: default::Renderable::new(&device, &swapchain_descriptor, &queue).await?,
            clipmap: clipmap::Renderable::new(&device, &swapchain_descriptor, &queue).await?,
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
    pub fn build_glyph_brush(device: &wgpu::Device, texture_format: wgpu::TextureFormat) -> wgpu_glyph::GlyphBrush<()> {
        let font = wgpu_glyph::ab_glyph::FontArc::try_from_slice(include_bytes!("../JetBrainsMono-Regular.ttf")).expect("Can not load font");
        let glyph_brush = wgpu_glyph::GlyphBrushBuilder::using_font(font).build(&device, texture_format);
        glyph_brush
    }

    pub async fn new(window: &Window) -> Result<Self> {
        // from here device creation and surface swapchain
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface =  unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface) }).await;
        let adapter = match adapter {
            Some(adapter) => adapter,
            None => { return Err(GraphicsError::RequestAdapter); },
        };
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor
        { features: Default::default(), limits: Default::default(), shader_validation: false }, None).await.unwrap();

        let sc_descriptor = wgpu::SwapChainDescriptor{
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
        self.depth_texture = texture::Texture::create_depth_texture(&self.device, &self.sc_descriptor);
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_descriptor);
    }
}

pub fn render_loop(renderables: &Renderables, device: &wgpu::Device, queue: &wgpu::Queue, target: &wgpu::TextureView, depth_attachment: &wgpu::TextureView) {
    let mut render_pass_creation_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    let mut renderable_encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut game_render_pass = render_pass_creation_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[
                wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: target,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0
                    }), store: true }
                }
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment: &depth_attachment,
                depth_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(1.0), store: true }),
                stencil_ops: Some(wgpu::Operations { load: wgpu::LoadOp::Clear(0), store: true }),
            }),
        });
        renderables.default.render(&device, queue, &mut renderable_encoder, &mut game_render_pass);
        renderables.clipmap.render(&device, queue, &mut renderable_encoder, &mut game_render_pass);
    }
    {
        let mut ui_render_pass = render_pass_creation_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[
                wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: target,
                    resolve_target: None,
                    ops: wgpu::Operations { load: wgpu::LoadOp::Load, store: true },
                }
            ],
            depth_stencil_attachment: None
        });
        renderables.ui.render(&device, queue, &mut renderable_encoder, &mut ui_render_pass);
    }
    let mut command_buffers = Vec::new();
    command_buffers.push(render_pass_creation_encoder.finish());
    command_buffers.push(renderable_encoder.finish());
    queue.submit(command_buffers);
}
