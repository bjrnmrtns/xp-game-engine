use crate::renderer::error::RendererError;
use winit::window::Window;

pub struct Renderer {
    surface: wgpu::Surface,
    pub device: wgpu::Device,
    queue: wgpu::Queue,
    pub swap_chain_descriptor: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
}

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self, RendererError> {
        let width = window.inner_size().width;
        let height = window.inner_size().width;
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
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_descriptor);
        Ok(Self {
            surface,
            device,
            queue,
            swap_chain_descriptor,
            swap_chain,
        })
    }

    pub async fn resize(&mut self, width: u32, height: u32) {
        self.swap_chain_descriptor.width = width;
        self.swap_chain_descriptor.height = height;
        self.swap_chain = self
            .device
            .create_swap_chain(&self.surface, &self.swap_chain_descriptor);
    }
}
