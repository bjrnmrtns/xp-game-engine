use crate::renderer::{depth_texture::DepthTexture, error::RendererError, Pipeline};
use winit::window::Window;

pub struct Renderer {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    queue: wgpu::Queue,
    pub swap_chain_descriptor: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    depth_texture: DepthTexture,
}

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self, RendererError> {
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
                return Err(RendererError::RequestAdapter);
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

        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let depth_texture = DepthTexture::create_depth_texture(&device, &swap_chain_descriptor);
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);
        Ok(Self {
            surface,
            device,
            queue,
            swap_chain_descriptor,
            swap_chain,
            depth_texture,
        })
    }

    pub async fn resize(&mut self, width: u32, height: u32) {
        self.swap_chain_descriptor.width = width;
        self.swap_chain_descriptor.height = height;
        self.depth_texture =
            DepthTexture::create_depth_texture(&self.device, &self.swap_chain_descriptor);
        self.swap_chain = self
            .device
            .create_swap_chain(&self.surface, &self.swap_chain_descriptor);
    }

    pub fn render(&mut self, pipeline: &Pipeline) {
        let target = &self
            .swap_chain
            .get_current_frame()
            .expect("Could not get next frame texture_view")
            .output
            .view;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
        }
        self.queue.submit(std::iter::once(encoder.finish()));
    }
}
