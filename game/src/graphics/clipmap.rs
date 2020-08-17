use crate::graphics::{Drawable, texture, create_drawable_from};
use nalgebra_glm::{Mat4, identity, Vec3, vec3};
use wgpu::{Device, BindingResource, BindGroupLayoutEntry, TextureViewDimension, RenderPass, CommandEncoder};
use crate::graphics::error::GraphicsError;
use crate::graphics;

type Result<T> = std::result::Result<T, GraphicsError>;

const CM_K: u32 = 8;
const CM_N: u32 = 255;
const CM_M: u32 = (CM_N + 1) / 4;
const CM_P: u32 = 3; // (CLIPMAP_N - 1) - ((CLIPMAP_M - 1) * 4) + 1 -> always 3
const CM_M_SIZE: u32 = CM_M - 1;
const CM_P_SIZE: u32 = CM_P - 1;
const CM_TEXTURE_SIZE: u32 = CM_N + 1;
const CM_TEXTURE_SIZE_HALF: u32 = CM_TEXTURE_SIZE / 2;
const CM_1M: u32 = CM_M_SIZE;
const CM_2M: u32 = CM_M_SIZE + CM_M_SIZE;
const CM_2M1P: u32 = CM_M_SIZE + CM_M_SIZE + CM_P_SIZE;
const CM_3M1P: u32 = CM_M_SIZE + CM_M_SIZE + CM_M_SIZE + CM_P_SIZE;

const CM_OFFSETS_MXM: [[u32; 2]; 12] = [[0, 0], [CM_1M, 0], [CM_2M1P, 0], [CM_3M1P, 0], // instances [0..12) -> mxm
                                         [0, CM_1M], [CM_3M1P, CM_1M],
                                         [0, CM_2M1P], [CM_3M1P, CM_2M1P],
                                         [0, CM_3M1P], [CM_1M, CM_3M1P], [CM_2M1P, CM_3M1P], [CM_3M1P, CM_3M1P],];

const CM_OFFSETS_MXP: [[u32; 2]; 2] = [[0, CM_2M], [CM_3M1P, CM_2M],]; // instances [14..16) -> mxp
const CM_OFFSETS_PXM: [[u32; 2]; 2] = [[CM_2M, 0], [CM_2M, CM_3M1P],]; // instances [12..14) -> pxm
const CM_OFFSET_NXN: [u32; 2] = [0, 0];
const CM_MAX_LEVELS: u32 = 7;
const CM_INSTANCE_SIZE_ONE_MXM: u32 = 12;
const CM_INSTANCE_SIZE_ONE_MXP: u32 = 2;
const CM_INSTANCE_SIZE_ONE_PXM: u32 = 2;
const CM_INSTANCE_SIZE_ONE_NXN: u32 = 1;
const CM_INSTANCE_SIZE_MXM: u32 = CM_INSTANCE_SIZE_ONE_MXM * CM_MAX_LEVELS;
const CM_INSTANCE_SIZE_MXP: u32 = CM_INSTANCE_SIZE_ONE_MXP * CM_MAX_LEVELS;
const CM_INSTANCE_SIZE_PXM: u32 = CM_INSTANCE_SIZE_ONE_PXM * CM_MAX_LEVELS;

#[allow(non_snake_case)]
pub fn create_clipmap_storage_texture(device: &wgpu::Device, N: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width: N,
            height: N,
            depth: 1
        },
        array_layer_count: 1,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::R32Float,
        usage: wgpu::TextureUsage::STORAGE | wgpu::TextureUsage::COPY_DST,
    })
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub p: [i32; 2],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Int2,
                },
            ]
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Instance {
    pub offset: [u32;2],
    pub level: u32,
    pub padding: u32,
}

unsafe impl bytemuck::Pod for Instance {}
unsafe impl bytemuck::Zeroable for Instance {}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniforms {
    pub projection: Mat4,
    pub view: Mat4,
    pub camera_position: Vec3,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

pub struct Renderable {
    pub drawables: Vec<Drawable>,
    pub uniforms_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
    pub texture: wgpu::Texture,

    uniforms: Uniforms,
    clipmap_data: Vec<f32>,
    clipmap_full: Drawable,
    clipmap_ring_mxm: Drawable,
    clipmap_ring_pxm: Drawable,
    clipmap_ring_mxp: Drawable,
}

impl Renderable {
    pub async fn new(device: &Device, sc_descriptor: &wgpu::SwapChainDescriptor, _queue: &wgpu::Queue) -> Result<Self> {
        let vs_spirv = glsl_to_spirv::compile(include_str!("../shader_clipmap.vert"), glsl_to_spirv::ShaderType::Vertex)?;
        let fs_spirv = glsl_to_spirv::compile(include_str!("../shader_clipmap.frag"), glsl_to_spirv::ShaderType::Fragment)?;
        let vs_data = wgpu::read_spirv(vs_spirv)?;
        let fs_data = wgpu::read_spirv(fs_spirv)?;
        let vs_module = device.create_shader_module(&vs_data);
        let fs_module = device.create_shader_module(&fs_data);

        let uniforms = Uniforms { projection: identity(), view: identity(), camera_position: vec3(0.0, 0.0, 0.0) };

        let uniform_buffer = device.create_buffer_with_data(bytemuck::cast_slice(&[uniforms]),
                                                            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST);
        let mut instances: Vec<Instance> = Vec::new();
        for level in 0..CM_MAX_LEVELS {
            instances.extend(CM_OFFSETS_MXM.iter().map(|offset| Instance { offset: offset.clone(), level, padding: 0 } ));
        }
        for level in 0..CM_MAX_LEVELS {
            instances.extend(CM_OFFSETS_MXP.iter().map(|offset| Instance { offset: offset.clone(), level, padding: 0 } ));
        }
        for level in 0..CM_MAX_LEVELS {
            instances.extend(CM_OFFSETS_PXM.iter().map(|offset| Instance { offset: offset.clone(), level, padding: 0 } ));
        }
        for level in 0..CM_MAX_LEVELS {
            instances.push(Instance { offset: CM_OFFSET_NXN, level, padding: 0 } );
        }
        let instance_buffer = device.create_buffer_with_data(bytemuck::cast_slice(instances.as_slice()),
                                                             wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::StorageTexture {
                        component_type: wgpu::TextureComponentType::Float,
                        format: wgpu::TextureFormat::R32Float,
                        dimension: TextureViewDimension::D2,
                        readonly: true
                    },
                },
            ],
            label: None,
        });

        let texture = create_clipmap_storage_texture(&device, CM_TEXTURE_SIZE as u32);
        assert_eq!(CM_N, (2 as u32).pow(CM_K) - 1);
        let (v, i) = create_grid(CM_N, CM_N);
        let clipmap_full = create_drawable_from(&device, (v.as_slice(), i.as_slice()));
        let (v, i) = create_grid(CM_M, CM_M);
        let clipmap_ring_mxm = create_drawable_from(&device, (v.as_slice(), i.as_slice()));
        let (v, i) = create_grid(CM_P, CM_M);
        let clipmap_ring_pxm = create_drawable_from(&device, (v.as_slice(), i.as_slice()));
        let (v, i) = create_grid(CM_M, CM_P);
        let clipmap_ring_mxp = create_drawable_from(&device, (v.as_slice(), i.as_slice()));

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
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
                        range: 0..std::mem::size_of_val(instances.as_slice()) as wgpu::BufferAddress,
                    }
                },
                wgpu::Binding {
                    binding: 2,
                    resource: BindingResource::TextureView(&texture.create_default_view()),
                },
            ],
            label: None,
        });
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
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
            primitive_topology: wgpu::PrimitiveTopology::LineList,
            depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                format: texture::Texture::DEPTH_FORMAT,
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

        Ok(Self {
            clipmap_full,
            clipmap_ring_mxm,
            clipmap_ring_pxm,
            clipmap_ring_mxp,
            drawables: Vec::new(),
            uniforms_buffer: uniform_buffer,
            instance_buffer,
            bind_group: bind_group,
            render_pipeline,
            texture,
            uniforms,
            clipmap_data: Vec::new(),
        })
    }

    pub fn update(&mut self, uniforms: Uniforms) {
        let sine = Sine {};
        self.uniforms = uniforms;
        self.clipmap_data = create_heightmap(&[uniforms.camera_position.x as i32, uniforms.camera_position.z as i32], &sine);
    }
}

pub fn create_grid(size_x: u32, size_z: u32) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices: Vec<Vertex> = Vec::new();
    for z in 0..size_z {
        for x in 0..size_x {
            vertices.push(Vertex {
                p: [x as i32, z as i32],
            })
        }
    }
    let mut indices: Vec<u32> = Vec::new();
    for z in 0..size_z-1 {
        for x in 0..size_x-1 {
            let i0 = x + z * size_x;
            let i1 = i0 + 1;
            let i2 = x + (z + 1) * size_x;
            let i3 = i2 + 1;
            indices.extend_from_slice(&[i0, i2, i2, i1, i1, i0, i1, i2, i2, i3, i3, i1]); // line_strip -> wireframe, indexbuffer for filled is remove even indices
        }
    }
    (vertices, indices)
}

impl graphics::Renderable for Renderable {
    fn render<'a, 'b>(&'a self, device: &Device, encoder: &mut CommandEncoder, render_pass: &'b mut RenderPass<'a>) where 'a: 'b {
        let uniforms_bufer = device.create_buffer_with_data(bytemuck::cast_slice(&[self.uniforms]), wgpu::BufferUsage::COPY_SRC);
        let height_map_data_buffer = device.create_buffer_with_data(bytemuck::cast_slice(self.clipmap_data.as_slice()), wgpu::BufferUsage::COPY_SRC);
        encoder.copy_buffer_to_texture(wgpu::BufferCopyView {
            buffer: &height_map_data_buffer,
            offset: 0,
            bytes_per_row: CM_TEXTURE_SIZE * 4,
            rows_per_image: CM_TEXTURE_SIZE
        }, wgpu::TextureCopyView{
            texture: &self.texture,
            mip_level: 0,
            array_layer: 0,
            origin: wgpu::Origin3d{
                x: 0,
                y: 0,
                z: 1
            }
        }, wgpu::Extent3d{
            width: CM_TEXTURE_SIZE,
            height: CM_TEXTURE_SIZE,
            depth: 1
        });

        encoder.copy_buffer_to_buffer(&uniforms_bufer, 0, &self.uniforms_buffer, 0, std::mem::size_of_val(&self.uniforms) as u64);
        render_pass.set_pipeline(&self.render_pipeline);
        let start_ring_level = 1;
        let full_level = start_ring_level - 1;

        let end_mxm = CM_INSTANCE_SIZE_MXM;
        let end_mxp = end_mxm + CM_INSTANCE_SIZE_MXP;
        let end_pxm = end_mxp + CM_INSTANCE_SIZE_PXM;
        render_pass.set_vertex_buffer(0, &self.clipmap_ring_mxm.vertex_buffer, 0, 0);
        render_pass.set_index_buffer(&self.clipmap_ring_mxm.index_buffer, 0, 0);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw_indexed(0..self.clipmap_ring_mxm.index_buffer_len, 0, start_ring_level * CM_INSTANCE_SIZE_ONE_MXM..end_mxm);

        render_pass.set_vertex_buffer(0, &self.clipmap_ring_mxp.vertex_buffer, 0, 0);
        render_pass.set_index_buffer(&self.clipmap_ring_mxp.index_buffer, 0, 0);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw_indexed(0..self.clipmap_ring_mxp.index_buffer_len, 0, end_mxm + start_ring_level * CM_INSTANCE_SIZE_ONE_MXP..end_mxp);

        render_pass.set_vertex_buffer(0, &self.clipmap_ring_pxm.vertex_buffer, 0, 0);
        render_pass.set_index_buffer(&self.clipmap_ring_pxm.index_buffer, 0, 0);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw_indexed(0..self.clipmap_ring_pxm.index_buffer_len, 0, end_mxp + start_ring_level * CM_INSTANCE_SIZE_ONE_PXM..end_pxm);

        render_pass.set_vertex_buffer(0, &self.clipmap_full.vertex_buffer, 0, 0);
        render_pass.set_index_buffer(&self.clipmap_full.index_buffer, 0, 0);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw_indexed(0..self.clipmap_full.index_buffer_len, 0, end_pxm + full_level * CM_INSTANCE_SIZE_ONE_NXN..(end_pxm + full_level * CM_INSTANCE_SIZE_ONE_NXN) + CM_INSTANCE_SIZE_ONE_NXN);
    }
}

struct Sine;

impl graphics::clipmap::Generator for Sine {
    fn generate(&self, pos: [f32; 2]) -> f32 {
        //(pos[0].sin()  + pos[1].cos()) / 4.0
        0.0
    }
}

pub trait Generator {
    fn generate(&self, pos: [f32; 2]) -> f32;
}

fn create_heightmap<T: Generator>(pos: &[i32; 2], generator: &T) -> Vec<f32> {
    const N: i32 = CM_TEXTURE_SIZE as i32;
    const N_HALF: i32 = CM_TEXTURE_SIZE_HALF as i32;
    let mut heightmap = vec!(0.0; (N * N) as usize);
    for z in pos[1]-N_HALF..pos[1]+N_HALF {
        for x in pos[0]-N_HALF..pos[0]+N_HALF {
            let height = generator.generate([x as f32, z as f32]);
            heightmap[x as usize % CM_TEXTURE_SIZE as usize + (z as usize % CM_TEXTURE_SIZE as usize) * CM_TEXTURE_SIZE as usize] = height;
        }
    }
    heightmap
}

