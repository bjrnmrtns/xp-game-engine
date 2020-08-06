use winit::window::Window;
use nalgebra_glm::*;
use crate::graphics::error::GraphicsError;
use crate::graphics::default_renderer::{Vertex};

pub mod default_renderer;
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

pub struct Drawable {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_buffer_len: u32,
}

pub struct Text
{
    pub pos: (f32, f32),
    pub text: String,
    pub font_size: f32,
    pub color: [u8; 4],
}


pub struct Graphics {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_descriptor: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    depth_texture: texture::Texture,
    ui_renderer: ui::Renderer,
    renderer: default_renderer::Renderer,
    clipmap_renderer: clipmap::Renderer,
    window_size: winit::dpi::PhysicalSize<u32>,
}

impl Graphics {
    pub fn build_glyph_brush(device: &wgpu::Device, texture_format: wgpu::TextureFormat) -> wgpu_glyph::GlyphBrush<()> {
        let font = wgpu_glyph::ab_glyph::FontArc::try_from_slice(include_bytes!("../JetBrainsMono-Regular.ttf")).expect("Can not load font");
        let glyph_brush = wgpu_glyph::GlyphBrushBuilder::using_font(font).build(&device, texture_format);
        glyph_brush
    }

    pub async fn new(window: &Window, ui_mesh: Mesh<ui::Vertex>) -> Result<Self> {
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
        let ui_renderer = ui::Renderer::new(&device, &sc_descriptor, &queue, ui_mesh).await?;
        let renderer= default_renderer::Renderer::new(&device, &sc_descriptor).await?;
        let clipmap_renderer= clipmap::Renderer::new(&device, &sc_descriptor, &queue).await?;

        Ok(Self {
            surface,
            device,
            queue,
            sc_descriptor,
            swap_chain,
            depth_texture,
            renderer,
            ui_renderer,
            clipmap_renderer,
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

    pub fn add_clipmap(&mut self, vertices: &Vec<clipmap::Vertex>, indices: &Vec<u32>) {
        let vertex_buffer = self.device.create_buffer_with_data(bytemuck::cast_slice(&vertices), wgpu::BufferUsage::VERTEX);
        let index_buffer = self.device.create_buffer_with_data(bytemuck::cast_slice(&indices), wgpu::BufferUsage::INDEX);
        self.clipmap_renderer.drawables.push(Drawable { vertex_buffer, index_buffer, index_buffer_len: indices.len() as u32, });
    }

    pub fn create_drawable_from_mesh(&mut self, mesh: &Mesh<Vertex>) -> usize {
        let vertex_buffer = self.device.create_buffer_with_data(bytemuck::cast_slice(&mesh.vertices), wgpu::BufferUsage::VERTEX);
        let index_buffer = self.device.create_buffer_with_data(bytemuck::cast_slice(&mesh.indices), wgpu::BufferUsage::INDEX);
        self.renderer.drawables.push(Drawable { vertex_buffer, index_buffer, index_buffer_len: mesh.indices.len() as u32, });
        self.renderer.drawables.len() - 1
    }

    pub fn create_drawable_from_mesh2(&mut self, mesh: &Mesh<Vertex>) -> Drawable {
        let vertex_buffer = self.device.create_buffer_with_data(bytemuck::cast_slice(&mesh.vertices), wgpu::BufferUsage::VERTEX);
        let index_buffer = self.device.create_buffer_with_data(bytemuck::cast_slice(&mesh.indices), wgpu::BufferUsage::INDEX);
        Drawable { vertex_buffer, index_buffer, index_buffer_len: mesh.indices.len() as u32, }
    }

    pub async fn render(&mut self, model_player: Mat4, model_terrain: Mat4, model_axis: Mat4, view: Mat4, render_ui: bool, ui_mesh: Option<(Mesh<ui::Vertex>, Vec<Text>)>, camera_position: Vec3) {
        let frame = self.swap_chain.get_next_texture().expect("failed to get next texture");
        let ui_projection = ortho(0.0, self.sc_descriptor.width as f32, 0.0, self.sc_descriptor.height as f32, -1.0, 1.0);
        let projection = perspective(self.sc_descriptor.width as f32 / self.sc_descriptor.height as f32,45.0, 0.1, 100.0);
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        self.renderer.update(&mut encoder, &self.device, projection.clone() as Mat4, view.clone() as Mat4, model_player, model_terrain, model_axis);
        self.clipmap_renderer.update(&mut encoder, &self.device, projection.clone() as Mat4, view.clone() as Mat4, camera_position.clone() as Vec3);
        self.ui_renderer.update(&mut encoder, &self.device, ui_projection.clone() as Mat4, ui_mesh);

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

            diffuse_scene_pass.set_pipeline(&self.renderer.render_pipeline);

            diffuse_scene_pass.set_vertex_buffer(0, &self.renderer.drawables[0].vertex_buffer, 0, 0);
            diffuse_scene_pass.set_index_buffer(&self.renderer.drawables[0].index_buffer, 0, 0);
            diffuse_scene_pass.set_bind_group(0, &self.renderer.uniform_bind_group, &[]);
            diffuse_scene_pass.draw_indexed(0..self.renderer.drawables[0].index_buffer_len, 0, 0..1);

            diffuse_scene_pass.set_vertex_buffer(0, &self.renderer.drawables[1].vertex_buffer, 0, 0);
            diffuse_scene_pass.set_index_buffer(&self.renderer.drawables[1].index_buffer, 0, 0);
            diffuse_scene_pass.set_bind_group(0, &self.renderer.uniform_bind_group, &[]);
            diffuse_scene_pass.draw_indexed(0..self.renderer.drawables[1].index_buffer_len, 0, 1..2);

            diffuse_scene_pass.set_pipeline(&self.clipmap_renderer.render_pipeline);
            diffuse_scene_pass.set_vertex_buffer(0, &self.clipmap_renderer.drawables[0].vertex_buffer, 0, 0);
            diffuse_scene_pass.set_index_buffer(&self.clipmap_renderer.drawables[0].index_buffer, 0, 0);
            diffuse_scene_pass.set_bind_group(0, &self.clipmap_renderer.bind_group, &[]);
            diffuse_scene_pass.draw_indexed(0..self.clipmap_renderer.drawables[0].index_buffer_len, 0, 0..1);
        }

        if render_ui
        {
            let mut ui_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
            ui_pass.set_pipeline(&self.ui_renderer.render_pipeline);
            ui_pass.set_vertex_buffer(0, &self.ui_renderer.drawable.vertex_buffer, 0, 0);
            ui_pass.set_index_buffer(&self.ui_renderer.drawable.index_buffer, 0, 0);
            ui_pass.set_bind_group(0, &self.ui_renderer.uniform_bind_group, &[]);
            ui_pass.set_bind_group(1, &self.ui_renderer.texture_bind_group, &[]);
            ui_pass.draw_indexed(0..self.ui_renderer.drawable.index_buffer_len, 0, 0..1);
        }
        if render_ui {
            self.ui_renderer.glyph_brush.draw_queued(&self.device, &mut encoder, &frame.view, self.sc_descriptor.width, self.sc_descriptor.height,).expect("Cannot draw glyph_brush");
        }
        self.queue.submit(&[encoder.finish()]);
    }
}
