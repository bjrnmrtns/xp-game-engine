use wgpu::*;

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

        Self { texture, view, sampler, }
    }

    #[allow(non_snake_case)]
    pub fn create_clipmap_texture(device: &wgpu::Device, N: u32) -> (Self, wgpu::CommandEncoder) {
        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width: N,
                height: N,
                depth: 1
            },
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::R32Float,
            usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
        });
        let mut data: Vec<f32> = vec![2.0;(N * N) as usize]; // should be all white
        data[8] = 0.0;
        let buffer = device.create_buffer_with_data(bytemuck::cast_slice(data.as_slice()), BufferUsage::COPY_SRC);
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {label: None});
        encoder.copy_buffer_to_texture(
            BufferCopyView {
                buffer: &buffer,
                offset: 0,
                bytes_per_row: N * 4,
                rows_per_image: N,
            },
            TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: Origin3d { x: 0, y: 0, z: 0 },
            },
            Extent3d {
                width: N,
                height: N,
                depth: 1,
            }
        );
        let view = texture.create_default_view();
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::Repeat,
            address_mode_v: AddressMode::Repeat,
            address_mode_w: AddressMode::Repeat,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: CompareFunction::Never,
        });

        (Self {
            texture,
            view,
            sampler
        }, encoder)
}

pub fn create_ui_texture(device: &wgpu::Device) -> (Self, wgpu::CommandEncoder) {
        let texture = device.create_texture(&TextureDescriptor {
            label: None,
            size: Extent3d {
                width: 100,
                height: 100,
                depth: 1,
            },
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
        });
        //TODO: copy data into texture from argument
        let data: Vec<u8> = vec![255;100 * 100 * 4]; // should be all white
        let buffer = device.create_buffer_with_data(data.as_slice(), BufferUsage::COPY_SRC);
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {label: None});
        encoder.copy_buffer_to_texture(
            BufferCopyView {
                buffer: &buffer,
                offset: 0,
                bytes_per_row: 100 * 4,
                rows_per_image: 100,
            },
            TextureCopyView {
                texture: &texture,
                mip_level: 0,
                array_layer: 0,
                origin: Origin3d { x: 0, y: 0, z: 0 },
            },
            Extent3d {
                width: 100,
                height: 100,
                depth: 1,
            }
        );
        let view = texture.create_default_view();

        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: CompareFunction::Always,
        });
        (Self {
            texture,
            view,
            sampler,
        }, encoder)
    }
}

