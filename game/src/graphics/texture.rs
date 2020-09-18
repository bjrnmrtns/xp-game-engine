use wgpu::*;
use wgpu::util::DeviceExt;

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float; // 1.
    pub fn create_depth_texture(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor) -> Self {
        let size = wgpu::Extent3d { // 2.
            width: sc_desc.width,
            height: sc_desc.height,
            depth: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT // 3.
                | wgpu::TextureUsage::SAMPLED
                | wgpu::TextureUsage::COPY_SRC,
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&Default::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor { // 4.
            label: None,
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: Some(wgpu::CompareFunction::LessEqual), // 5.
            anisotropy_clamp: None
        });

        Self { texture, view, sampler, }
    }

    pub fn create_ui_texture(device: &wgpu::Device) -> (Self, wgpu::CommandEncoder) {
        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width: 256,
                height: 256,
                depth: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
        });
        //TODO: copy data into texture from argument
        let data: Vec<u8> = vec![255;256 * 256 * 4]; // should be all white
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(data.as_slice()),
            usage: wgpu::BufferUsage::COPY_SRC
        });
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {label: None});
        encoder.copy_buffer_to_texture(
            BufferCopyView {
                buffer: &buffer,
                layout: wgpu::TextureDataLayout {
                    offset: 0,
                    bytes_per_row: 256 * 4,
                    rows_per_image: 256,
                }
            },
            TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d { x: 0, y: 0, z: 0 },
            },
            Extent3d {
                width: 256,
                height: 256,
                depth: 1,
            }
        );
        let view = texture.create_view(&Default::default());

        let sampler = device.create_sampler(&SamplerDescriptor {
            label: None,
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: None,
            anisotropy_clamp: None
        });
        (Self {
            texture,
            view,
            sampler,
        }, encoder)
    }
}

