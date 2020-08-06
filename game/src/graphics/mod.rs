use winit::window::Window;
use crate::graphics::error::GraphicsError;

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

pub struct RenderPipelines {
    pub ui: ui::Pipeline,
    pub default: default::Pipeline,
    pub clipmap: clipmap::Pipeline,
}

impl RenderPipelines {
    pub async fn new(device: &wgpu::Device, queue: &wgpu::Queue, swapchain_descriptor: &wgpu::SwapChainDescriptor) -> Result<Self> {
        Ok(Self {
            ui: ui::Pipeline::new(&device, &swapchain_descriptor, &queue).await?,
            default: default::Pipeline::new(&device, &swapchain_descriptor, &queue).await?,
            clipmap: clipmap::Pipeline::new(&device, &swapchain_descriptor, &queue).await?,
        })
    }
}

pub struct Graphics {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub sc_descriptor: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    depth_texture: texture::Texture,
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

pub async fn render_loop(graphics: &mut Graphics, render_pipelines: &mut RenderPipelines, render_ui: bool) {
    let frame = graphics.swap_chain.get_next_texture().expect("failed to get next texture");
    let mut encoder = graphics.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    render_pipelines.default.pre_render(&graphics.device, &mut encoder);
    render_pipelines.clipmap.pre_render(&graphics.device, &mut encoder);
    render_pipelines.ui.pre_render(&graphics.device, &mut encoder);

    // render with all renderers with respective render passes
    {
        let mut game_render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
                attachment: &graphics.depth_texture.view,
                depth_load_op: wgpu::LoadOp::Clear,
                depth_store_op: wgpu::StoreOp::Store,
                clear_depth: 1.0,
                stencil_load_op: wgpu::LoadOp::Clear,
                stencil_store_op: wgpu::StoreOp::Store,
                clear_stencil: 0,
            }),
        });

        game_render_pass.set_pipeline(&render_pipelines.default.render_pipeline);

        game_render_pass.set_vertex_buffer(0, &render_pipelines.default.drawables[0].vertex_buffer, 0, 0);
        game_render_pass.set_index_buffer(&render_pipelines.default.drawables[0].index_buffer, 0, 0);
        game_render_pass.set_bind_group(0, &render_pipelines.default.uniform_bind_group, &[]);
        game_render_pass.draw_indexed(0..render_pipelines.default.drawables[0].index_buffer_len, 0, 0..1);

        game_render_pass.set_vertex_buffer(0, &render_pipelines.default.drawables[1].vertex_buffer, 0, 0);
        game_render_pass.set_index_buffer(&render_pipelines.default.drawables[1].index_buffer, 0, 0);
        game_render_pass.set_bind_group(0, &render_pipelines.default.uniform_bind_group, &[]);
        game_render_pass.draw_indexed(0..render_pipelines.default.drawables[1].index_buffer_len, 0, 1..2);

        game_render_pass.set_pipeline(&render_pipelines.clipmap.render_pipeline);
        game_render_pass.set_vertex_buffer(0, &render_pipelines.clipmap.drawables[0].vertex_buffer, 0, 0);
        game_render_pass.set_index_buffer(&render_pipelines.clipmap.drawables[0].index_buffer, 0, 0);
        game_render_pass.set_bind_group(0, &render_pipelines.clipmap.bind_group, &[]);
        game_render_pass.draw_indexed(0..render_pipelines.clipmap.drawables[0].index_buffer_len, 0, 0..1);
    }

    if render_ui
    {
        let mut ui_render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[
                wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Load,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }
                }
            ],
            depth_stencil_attachment: None
        });
        ui_render_pass.set_pipeline(&render_pipelines.ui.render_pipeline);
        if let Some(drawable) = &render_pipelines.ui.drawable {
            ui_render_pass.set_vertex_buffer(0, &drawable.vertex_buffer, 0, 0);
            ui_render_pass.set_index_buffer(&drawable.index_buffer, 0, 0);
            ui_render_pass.set_bind_group(0, &render_pipelines.ui.uniform_bind_group, &[]);
            ui_render_pass.set_bind_group(1, &render_pipelines.ui.texture_bind_group, &[]);
            ui_render_pass.draw_indexed(0..drawable.index_buffer_len, 0, 0..1);
        }
    }
    if render_ui {
        render_pipelines.ui.glyph_brush.draw_queued(&graphics.device, &mut encoder, &frame.view, graphics.sc_descriptor.width, graphics.sc_descriptor.height,).expect("Cannot draw glyph_brush");
    }
    graphics.queue.submit(&[encoder.finish()]);
}
