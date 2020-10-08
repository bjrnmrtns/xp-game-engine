use crate::graphics::error::GraphicsError;
use crate::graphics::{create_drawable_from, texture, Drawable};
use crate::terrain;
use crate::terrain::Generator;
use nalgebra_glm::{identity, vec3, Mat4, Vec3};
use std::io::Read;
use wgpu::util::DeviceExt;
use wgpu::{BindingResource, Device, RenderPass, TextureViewDimension};

type Result<T> = std::result::Result<T, GraphicsError>;

const WIRE_FRAME: bool = false;

const CM_K: u32 = 7;
const CM_N: u32 = 127;
const CM_ELEMENT_SIZE: u32 = 4; // number of bytes of an element (now height(f32) -> total: 4

const CM_UNIT_SIZE_SMALLEST: f32 = 2.0;
const CM_M: u32 = (CM_N + 1) / 4;
const CM_P: u32 = 3; // (CLIPMAP_N - 1) - ((CLIPMAP_M - 1) * 4) + 1 -> always 3
const CM_M_SIZE: u32 = CM_M - 1;
const CM_P_SIZE: u32 = CM_P - 1;
const CM_INTERIOR_SIZE: u32 = CM_M * 2 + 1;
const CM_TEXTURE_SIZE: u32 = CM_N + 1;
const CM_1M: u32 = CM_M_SIZE;
const CM_2M: u32 = CM_M_SIZE + CM_M_SIZE;
const CM_2M1P: u32 = CM_M_SIZE + CM_M_SIZE + CM_P_SIZE;
const CM_3M1P: u32 = CM_M_SIZE + CM_M_SIZE + CM_M_SIZE + CM_P_SIZE;
const CM_4M1P: u32 = CM_M_SIZE + CM_M_SIZE + CM_M_SIZE + CM_M_SIZE + CM_P_SIZE;

const CM_OFFSETS_MXM: [[u32; 2]; 12] = [
    [0, 0],
    [CM_1M, 0],
    [CM_2M1P, 0],
    [CM_3M1P, 0], // instances [0..12) -> mxm
    [0, CM_1M],
    [CM_3M1P, CM_1M],
    [0, CM_2M1P],
    [CM_3M1P, CM_2M1P],
    [0, CM_3M1P],
    [CM_1M, CM_3M1P],
    [CM_2M1P, CM_3M1P],
    [CM_3M1P, CM_3M1P],
];

const CM_OFFSETS_MXP: [[u32; 2]; 2] = [[0, CM_2M], [CM_3M1P, CM_2M]]; // instances [14..16) -> mxp
const CM_OFFSETS_PXM: [[u32; 2]; 2] = [[CM_2M, 0], [CM_2M, CM_3M1P]]; // instances [12..14) -> pxm
const CM_OFFSETS_INTERIOR_H_BOTTOM: [u32; 2] = [CM_1M, CM_3M1P - 1];
const CM_OFFSETS_INTERIOR_H_TOP: [u32; 2] = [CM_1M, CM_1M];
const CM_OFFSETS_INTERIOR_V_LEFT: [u32; 2] = [CM_1M, CM_1M];
const CM_OFFSETS_INTERIOR_V_RIGHT: [u32; 2] = [CM_3M1P - 1, CM_1M];
const CM_OFFSETS_DEGENERATES_H_TOP: [u32; 2] = [0, 0];
const CM_OFFSETS_DEGENERATES_H_BOTTOM: [u32; 2] = [0, CM_4M1P];
const CM_OFFSETS_DEGENERATES_V_LEFT: [u32; 2] = [0, 0];
const CM_OFFSETS_DEGENERATES_V_RIGHT: [u32; 2] = [CM_4M1P, 0];
const CM_OFFSET_NXN: [u32; 2] = [0, 0];
const CM_MAX_LEVELS: u32 = 5;
const CM_INSTANCE_SIZE_ONE_MXM: u32 = 12;
const CM_INSTANCE_SIZE_ONE_MXP: u32 = 2;
const CM_INSTANCE_SIZE_ONE_PXM: u32 = 2;
const CM_INSTANCE_SIZE_ONE_INTERIOR: u32 = 1;
const CM_INSTANCE_SIZE_ONE_DEGENERATE: u32 = 1;
const CM_INSTANCE_SIZE_ONE_NXN: u32 = 1;
const CM_INSTANCE_SIZE_MXM: u32 = CM_INSTANCE_SIZE_ONE_MXM * CM_MAX_LEVELS;
const CM_INSTANCE_SIZE_MXP: u32 = CM_INSTANCE_SIZE_ONE_MXP * CM_MAX_LEVELS;
const CM_INSTANCE_SIZE_PXM: u32 = CM_INSTANCE_SIZE_ONE_PXM * CM_MAX_LEVELS;
const CM_INSTANCE_SIZE_NXN: u32 = CM_INSTANCE_SIZE_ONE_NXN * CM_MAX_LEVELS;
const CM_INSTANCE_SIZE_DEGENERATES: u32 = CM_INSTANCE_SIZE_ONE_DEGENERATE * CM_MAX_LEVELS;
const CM_INSTANCE_SIZE_INTERIOR: u32 = CM_INSTANCE_SIZE_ONE_INTERIOR * CM_MAX_LEVELS;
const BASE_OFFSET: u32 = (CM_N - 3) / 2;

#[allow(non_snake_case)]
pub fn create_clipmap_storage_texture(device: &wgpu::Device, N: u32) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width: N,
            height: N,
            depth: CM_MAX_LEVELS,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D3,
        format: wgpu::TextureFormat::R32Float,
        usage: wgpu::TextureUsage::STORAGE | wgpu::TextureUsage::COPY_DST,
    })
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub p: [i32; 3],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[wgpu::VertexAttributeDescriptor {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Int3,
            }],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Instance {
    pub offset: [u32; 2],
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
    pub uniforms_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub render_pipeline: wgpu::RenderPipeline,
    pub texture: wgpu::Texture,
    pub generator: Box<dyn Generator>,

    uniforms: Uniforms,
    pub clipmap_data: Clipmap,
    clipmap_full: Drawable,
    clipmap_ring_mxm: Drawable,
    clipmap_ring_pxm: Drawable,
    clipmap_ring_mxp: Drawable,
    clipmap_interior_h: Drawable,
    clipmap_interior_v: Drawable,
    clipmap_degenerates_h_top: Drawable,
    clipmap_degenerates_h_bottom: Drawable,
    clipmap_degenerates_v_left: Drawable,
    clipmap_degenerates_v_right: Drawable,
}

impl Renderable {
    pub async fn new(
        device: &Device,
        sc_descriptor: &wgpu::SwapChainDescriptor,
        _queue: &wgpu::Queue,
    ) -> Result<Self> {
        let (mut spirv_vs_bytes, mut spirv_fs_bytes) = (Vec::new(), Vec::new());
        match glsl_to_spirv::compile(
            include_str!("../shader_clipmap.vert"),
            glsl_to_spirv::ShaderType::Vertex,
        ) {
            Ok(mut spirv_vs_output) => {
                spirv_vs_output.read_to_end(&mut spirv_vs_bytes).unwrap();
            }
            Err(ref e) => {
                return Err(GraphicsError::from(e.clone()));
            }
        }
        match glsl_to_spirv::compile(
            include_str!("../shader_clipmap.frag"),
            glsl_to_spirv::ShaderType::Fragment,
        ) {
            Ok(mut spirv_vs_output) => {
                spirv_vs_output.read_to_end(&mut spirv_fs_bytes).unwrap();
            }
            Err(ref e) => {
                return Err(GraphicsError::from(e.clone()));
            }
        }
        let vs_module_source = wgpu::util::make_spirv(spirv_vs_bytes.as_slice());
        let fs_module_source = wgpu::util::make_spirv(spirv_fs_bytes.as_slice());
        let vs_module = device.create_shader_module(vs_module_source);
        let fs_module = device.create_shader_module(fs_module_source);

        let uniforms = Uniforms {
            projection: identity(),
            view: identity(),
            camera_position: vec3(0.0, 0.0, 0.0),
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let mut instances: Vec<Instance> = Vec::new();
        for level in 0..CM_MAX_LEVELS {
            instances.extend(CM_OFFSETS_MXM.iter().map(|offset| Instance {
                offset: offset.clone(),
                level,
                padding: 0,
            }));
        }
        for level in 0..CM_MAX_LEVELS {
            instances.extend(CM_OFFSETS_MXP.iter().map(|offset| Instance {
                offset: offset.clone(),
                level,
                padding: 1,
            }));
        }
        for level in 0..CM_MAX_LEVELS {
            instances.extend(CM_OFFSETS_PXM.iter().map(|offset| Instance {
                offset: offset.clone(),
                level,
                padding: 2,
            }));
        }
        for level in 0..CM_MAX_LEVELS {
            instances.push(Instance {
                offset: CM_OFFSET_NXN,
                level,
                padding: 0,
            });
        }

        for level in 0..CM_MAX_LEVELS {
            instances.push(Instance {
                offset: CM_OFFSETS_INTERIOR_H_BOTTOM,
                level,
                padding: 4,
            });
        }
        for level in 0..CM_MAX_LEVELS {
            instances.push(Instance {
                offset: CM_OFFSETS_INTERIOR_H_TOP,
                level,
                padding: 4,
            });
        }
        for level in 0..CM_MAX_LEVELS {
            instances.push(Instance {
                offset: CM_OFFSETS_INTERIOR_V_LEFT,
                level,
                padding: 4,
            });
        }
        for level in 0..CM_MAX_LEVELS {
            instances.push(Instance {
                offset: CM_OFFSETS_INTERIOR_V_RIGHT,
                level,
                padding: 4,
            });
        }

        for level in 0..CM_MAX_LEVELS {
            instances.push(Instance {
                offset: CM_OFFSETS_DEGENERATES_H_TOP,
                level,
                padding: 5,
            });
        }
        for level in 0..CM_MAX_LEVELS {
            instances.push(Instance {
                offset: CM_OFFSETS_DEGENERATES_H_BOTTOM,
                level,
                padding: 5,
            });
        }
        for level in 0..CM_MAX_LEVELS {
            instances.push(Instance {
                offset: CM_OFFSETS_DEGENERATES_V_LEFT,
                level,
                padding: 5,
            });
        }
        for level in 0..CM_MAX_LEVELS {
            instances.push(Instance {
                offset: CM_OFFSETS_DEGENERATES_V_RIGHT,
                level,
                padding: 5,
            });
        }
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(instances.as_slice()),
            usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::StorageBuffer {
                        dynamic: false,
                        min_binding_size: None,
                        readonly: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::StorageTexture {
                        format: wgpu::TextureFormat::Rgba32Float,
                        dimension: TextureViewDimension::D3,
                        readonly: true,
                    },
                    count: None,
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
        let (v, i) = create_grid(CM_INTERIOR_SIZE, 2);
        let clipmap_interior_h = create_drawable_from(&device, (v.as_slice(), i.as_slice()));
        let (v, i) = create_grid(2, CM_INTERIOR_SIZE);
        let clipmap_interior_v = create_drawable_from(&device, (v.as_slice(), i.as_slice()));
        let (v, i) = create_degenerates_top(CM_N);
        let clipmap_degenerates_h_top = create_drawable_from(&device, (v.as_slice(), i.as_slice()));
        let (v, i) = create_degenerates_bottom(CM_N);
        let clipmap_degenerates_h_bottom =
            create_drawable_from(&device, (v.as_slice(), i.as_slice()));
        let (v, i) = create_degenerates_left(CM_N);
        let clipmap_degenerates_v_left =
            create_drawable_from(&device, (v.as_slice(), i.as_slice()));
        let (v, i) = create_degenerates_right(CM_N);
        let clipmap_degenerates_v_right =
            create_drawable_from(&device, (v.as_slice(), i.as_slice()));

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        uniform_buffer.slice(0..std::mem::size_of_val(&uniforms) as u64),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer(
                        instance_buffer.slice(0..std::mem::size_of_val(&instances) as u64),
                    ),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::TextureView(
                        &texture.create_view(&Default::default()),
                    ),
                },
            ],
            label: None,
        });
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let primitive_topology = if WIRE_FRAME {
            wgpu::PrimitiveTopology::LineList
        } else {
            wgpu::PrimitiveTopology::TriangleList
        };

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                clamp_depth: false,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_descriptor.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            primitive_topology,
            depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                format: texture::Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilStateDescriptor {
                    front: wgpu::StencilStateFaceDescriptor::IGNORE,
                    back: wgpu::StencilStateFaceDescriptor::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
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
            clipmap_interior_h,
            clipmap_interior_v,
            clipmap_degenerates_h_top,
            clipmap_degenerates_h_bottom,
            clipmap_degenerates_v_left,
            clipmap_degenerates_v_right,
            uniforms_buffer: uniform_buffer,
            instance_buffer,
            bind_group: bind_group,
            render_pipeline,
            texture,
            uniforms,
            clipmap_data: Clipmap::new(CM_MAX_LEVELS),
            generator: Box::new(terrain::Fbm::new()),
        })
    }

    pub fn pre_render(&mut self, queue: &wgpu::Queue, uniforms: Uniforms) {
        self.uniforms = uniforms;
        self.clipmap_data.copy_regions.clear();
        for level in 0..CM_MAX_LEVELS {
            update_heightmap(
                &mut self.clipmap_data,
                [
                    self.uniforms.camera_position.x,
                    self.uniforms.camera_position.z,
                ],
                level,
                &*self.generator,
            );
        }
        queue.write_buffer(
            &self.uniforms_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );
        for copy_region in &self.clipmap_data.copy_regions {
            let offset_in_level = copy_region.x + (copy_region.y * CM_TEXTURE_SIZE);
            let begin_slice =
                (copy_region.level * CM_TEXTURE_SIZE * CM_TEXTURE_SIZE) + offset_in_level;
            let end_slice =
                begin_slice + copy_region.xlen + CM_TEXTURE_SIZE * (copy_region.ylen - 1);
            queue.write_texture(
                wgpu::TextureCopyView {
                    texture: &self.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: copy_region.x,
                        y: copy_region.y,
                        z: copy_region.level,
                    },
                },
                bytemuck::cast_slice(
                    &self.clipmap_data.data.as_slice()[begin_slice as usize..end_slice as usize],
                ),
                wgpu::TextureDataLayout {
                    offset: 0,
                    bytes_per_row: CM_TEXTURE_SIZE * CM_ELEMENT_SIZE,
                    rows_per_image: copy_region.ylen,
                },
                wgpu::Extent3d {
                    width: copy_region.xlen as u32,
                    height: copy_region.ylen as u32,
                    depth: 1,
                },
            );
        }
    }

    pub fn render<'a, 'b>(&'a self, render_pass: &'b mut RenderPass<'a>)
    where
        'a: 'b,
    {
        render_pass.set_pipeline(&self.render_pipeline);
        let start_ring_level = 1;
        let full_level = start_ring_level - 1;

        let end_mxm = CM_INSTANCE_SIZE_MXM;
        let end_mxp = end_mxm + CM_INSTANCE_SIZE_MXP;
        let end_pxm = end_mxp + CM_INSTANCE_SIZE_PXM;
        let end_nxn: u32 = end_pxm + CM_INSTANCE_SIZE_NXN;
        let end_interior_h_bottom: u32 = end_nxn + CM_INSTANCE_SIZE_INTERIOR;
        let end_interior_h_top: u32 = end_interior_h_bottom + CM_INSTANCE_SIZE_INTERIOR;
        let end_interior_v_left: u32 = end_interior_h_top + CM_INSTANCE_SIZE_INTERIOR;
        let end_interior_v_right: u32 = end_interior_v_left + CM_INSTANCE_SIZE_INTERIOR;
        let end_degen_h_top: u32 = end_interior_v_right + CM_INSTANCE_SIZE_DEGENERATES;
        let end_degen_h_bottom: u32 = end_degen_h_top + CM_INSTANCE_SIZE_DEGENERATES;
        let end_degen_v_left: u32 = end_degen_h_bottom + CM_INSTANCE_SIZE_DEGENERATES;
        let end_degen_v_right: u32 = end_degen_v_left + CM_INSTANCE_SIZE_DEGENERATES;

        render_pass.set_vertex_buffer(0, self.clipmap_ring_mxm.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.clipmap_ring_mxm.index_buffer.slice(..));
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw_indexed(
            0..self.clipmap_ring_mxm.index_buffer_len,
            0,
            start_ring_level * CM_INSTANCE_SIZE_ONE_MXM..end_mxm,
        );

        render_pass.set_vertex_buffer(0, self.clipmap_ring_mxp.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.clipmap_ring_mxp.index_buffer.slice(..));
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw_indexed(
            0..self.clipmap_ring_mxp.index_buffer_len,
            0,
            end_mxm + start_ring_level * CM_INSTANCE_SIZE_ONE_MXP..end_mxp,
        );

        render_pass.set_vertex_buffer(0, self.clipmap_ring_pxm.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.clipmap_ring_pxm.index_buffer.slice(..));
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw_indexed(
            0..self.clipmap_ring_pxm.index_buffer_len,
            0,
            end_mxp + start_ring_level * CM_INSTANCE_SIZE_ONE_PXM..end_pxm,
        );

        render_pass.set_vertex_buffer(0, self.clipmap_full.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.clipmap_full.index_buffer.slice(..));
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw_indexed(
            0..self.clipmap_full.index_buffer_len,
            0,
            end_pxm + full_level * CM_INSTANCE_SIZE_ONE_NXN
                ..(end_pxm + full_level * CM_INSTANCE_SIZE_ONE_NXN) + CM_INSTANCE_SIZE_ONE_NXN,
        );

        for level in start_ring_level..CM_MAX_LEVELS {
            //h_bottom
            if snap_diff(self.uniforms.camera_position.z, level - 1, level) < std::f32::EPSILON {
                let start_instance = end_nxn + level * CM_INSTANCE_SIZE_ONE_INTERIOR;
                render_pass.set_vertex_buffer(0, self.clipmap_interior_h.vertex_buffer.slice(..));
                render_pass.set_index_buffer(self.clipmap_interior_h.index_buffer.slice(..));
                render_pass.set_bind_group(0, &self.bind_group, &[]);
                render_pass.draw_indexed(
                    0..self.clipmap_interior_h.index_buffer_len,
                    0,
                    start_instance..start_instance + 1,
                );
            }
        }

        for level in start_ring_level..CM_MAX_LEVELS {
            //h_top
            if snap_diff(self.uniforms.camera_position.z, level - 1, level) > std::f32::EPSILON {
                let start_instance = end_interior_h_bottom + level * CM_INSTANCE_SIZE_ONE_INTERIOR;
                render_pass.set_vertex_buffer(0, self.clipmap_interior_h.vertex_buffer.slice(..));
                render_pass.set_index_buffer(self.clipmap_interior_h.index_buffer.slice(..));
                render_pass.set_bind_group(0, &self.bind_group, &[]);
                render_pass.draw_indexed(
                    0..self.clipmap_interior_h.index_buffer_len,
                    0,
                    start_instance..start_instance + 1,
                );
            }
        }

        for level in start_ring_level..CM_MAX_LEVELS {
            //v_left
            if snap_diff(self.uniforms.camera_position.x, level - 1, level) > std::f32::EPSILON {
                let start_instance = end_interior_h_top + level * CM_INSTANCE_SIZE_ONE_INTERIOR;
                render_pass.set_vertex_buffer(0, self.clipmap_interior_v.vertex_buffer.slice(..));
                render_pass.set_index_buffer(self.clipmap_interior_v.index_buffer.slice(..));
                render_pass.set_bind_group(0, &self.bind_group, &[]);
                render_pass.draw_indexed(
                    0..self.clipmap_interior_v.index_buffer_len,
                    0,
                    start_instance..start_instance + 1,
                );
            }
        }

        for level in start_ring_level..CM_MAX_LEVELS {
            //v_right
            if snap_diff(self.uniforms.camera_position.x, level - 1, level) < std::f32::EPSILON {
                let start_instance = end_interior_v_left + level * CM_INSTANCE_SIZE_ONE_INTERIOR;
                render_pass.set_vertex_buffer(0, self.clipmap_interior_v.vertex_buffer.slice(..));
                render_pass.set_index_buffer(self.clipmap_interior_v.index_buffer.slice(..));
                render_pass.set_bind_group(0, &self.bind_group, &[]);
                render_pass.draw_indexed(
                    0..self.clipmap_interior_v.index_buffer_len,
                    0,
                    start_instance..start_instance + 1,
                );
            }
        }

        render_pass.set_vertex_buffer(0, self.clipmap_degenerates_h_top.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.clipmap_degenerates_h_top.index_buffer.slice(..));
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw_indexed(
            0..self.clipmap_degenerates_h_top.index_buffer_len,
            0,
            end_interior_v_right + full_level * CM_INSTANCE_SIZE_ONE_DEGENERATE..end_degen_h_top,
        );

        render_pass.set_vertex_buffer(0, self.clipmap_degenerates_h_bottom.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.clipmap_degenerates_h_bottom.index_buffer.slice(..));
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw_indexed(
            0..self.clipmap_degenerates_h_bottom.index_buffer_len,
            0,
            end_degen_h_top + full_level * CM_INSTANCE_SIZE_ONE_DEGENERATE..end_degen_h_bottom,
        );

        render_pass.set_vertex_buffer(0, self.clipmap_degenerates_v_left.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.clipmap_degenerates_v_left.index_buffer.slice(..));
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw_indexed(
            0..self.clipmap_degenerates_v_left.index_buffer_len,
            0,
            end_degen_h_bottom + full_level * CM_INSTANCE_SIZE_ONE_DEGENERATE..end_degen_v_left,
        );

        render_pass.set_vertex_buffer(0, self.clipmap_degenerates_v_right.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.clipmap_degenerates_v_right.index_buffer.slice(..));
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw_indexed(
            0..self.clipmap_degenerates_v_right.index_buffer_len,
            0,
            end_degen_v_left + full_level * CM_INSTANCE_SIZE_ONE_DEGENERATE..end_degen_v_right,
        );
    }
}

pub fn create_degenerates_top(size: u32) -> (Vec<Vertex>, Vec<u32>) {
    assert!((size + 1) % 2 == 0);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for x in 0..size {
        vertices.push(Vertex {
            p: [x as i32, 0, 0],
        });
    }
    for x in (0..size - 1).step_by(2) {
        let i0 = x;
        let i1 = x + 1;
        let i2 = x + 2;
        if WIRE_FRAME {
            indices.extend_from_slice(&[i0, i1, i1, i2, i2, i0]);
        } else {
            indices.extend_from_slice(&[i0, i1, i2]);
        }
    }
    (vertices, indices)
}

pub fn create_degenerates_bottom(size: u32) -> (Vec<Vertex>, Vec<u32>) {
    assert!((size + 1) % 2 == 0);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for x in 0..size {
        vertices.push(Vertex {
            p: [x as i32, 0, 0],
        });
    }
    for x in (0..size - 1).step_by(2) {
        let i0 = x;
        let i1 = x + 2;
        let i2 = x + 1;
        if WIRE_FRAME {
            indices.extend_from_slice(&[i0, i1, i1, i2, i2, i0]);
        } else {
            indices.extend_from_slice(&[i0, i1, i2]);
        }
    }
    (vertices, indices)
}

pub fn create_degenerates_left(size: u32) -> (Vec<Vertex>, Vec<u32>) {
    assert!((size + 1) % 2 == 0);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for z in 0..size {
        vertices.push(Vertex {
            p: [0, z as i32, 0],
        });
    }
    for z in (0..size - 1).step_by(2) {
        let i0 = z;
        let i1 = z + 2;
        let i2 = z + 1;
        if WIRE_FRAME {
            indices.extend_from_slice(&[i0, i1, i1, i2, i2, i0]);
        } else {
            indices.extend_from_slice(&[i0, i1, i2]);
        }
    }
    (vertices, indices)
}

pub fn create_degenerates_right(size: u32) -> (Vec<Vertex>, Vec<u32>) {
    assert!((size + 1) % 2 == 0);
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for z in 0..size {
        vertices.push(Vertex {
            p: [0, z as i32, 0],
        });
    }
    for z in (0..size - 1).step_by(2) {
        let i0 = z;
        let i1 = z + 1;
        let i2 = z + 2;
        if WIRE_FRAME {
            indices.extend_from_slice(&[i0, i1, i1, i2, i2, i0]);
        } else {
            indices.extend_from_slice(&[i0, i1, i2]);
        }
    }
    (vertices, indices)
}

pub fn create_grid(size_x: u32, size_z: u32) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut index = 0;
    for z in 0..size_z - 1 {
        for x in 0..size_x - 1 {
            // first triangle, provoking vertex:
            vertices.push(Vertex {
                p: [x as i32, z as i32, 1],
            });
            vertices.push(Vertex {
                p: [(x + 1) as i32, z as i32, 1],
            });
            vertices.push(Vertex {
                p: [x as i32, (z + 1) as i32, 1],
            });
            // second triangle, provoking vertex:
            vertices.push(Vertex {
                p: [(x + 1) as i32, (z + 1) as i32, -1],
            });
            let i0 = index;
            let i1 = i0 + 1;
            let i2 = i1 + 1;
            let i3 = i2 + 1;
            index += 4;

            if WIRE_FRAME {
                indices.extend_from_slice(&[i0, i2, i2, i1, i1, i0, i3, i1, i1, i2, i2, i3]);
            } else {
                indices.extend_from_slice(&[i0, i2, i1, i3, i1, i2]);
            }
        }
    }
    (vertices, indices)
}

fn level_factor(level: u32) -> u32 {
    2u32.pow(level)
}

fn unit_size_for_level(level: u32) -> f32 {
    level_factor(level) as f32 * CM_UNIT_SIZE_SMALLEST
}

fn snap_diff(val: f32, level_a: u32, level_b: u32) -> f32 {
    snap_value_for_level(val, level_a) - snap_value_for_level(val, level_b)
}

fn snap_value_for_level(val: f32, level: u32) -> f32 {
    let snap_size = unit_size_for_level(level + 1);
    (val / snap_size).floor() * snap_size
}

fn snap_to_index_for_level(val: f32, level: u32) -> i32 {
    let snap_size = unit_size_for_level(level + 1);
    ((val / snap_size).floor() * 2.0) as i32
}

fn equal_coords(first: &[i32; 2], second: &[i32; 2]) -> bool {
    first[0] == second[0] && first[1] == second[1]
}

fn update_xrows(
    clipmap: &mut Clipmap,
    xrange: &std::ops::Range<i32>,
    zstart: i32,
    level: u32,
    generator: &dyn terrain::Generator,
) {
    let unit_size = unit_size_for_level(level);
    for z in zstart..zstart + CM_TEXTURE_SIZE as i32 {
        for x in xrange.clone() {
            let x_pos = x as f32 * unit_size;
            let z_pos = z as f32 * unit_size;
            let x_mod = x as u32 % CM_TEXTURE_SIZE;
            let z_mod = z as u32 % CM_TEXTURE_SIZE;
            //TODO: calculate normal of point, but expensive to call 4 times noise? -> first create heights, then derive rest
            clipmap.set_height(x_mod, z_mod, level, generator.generate([x_pos, z_pos]));
        }
    }
}

fn update_zrows(
    clipmap: &mut Clipmap,
    zrange: &std::ops::Range<i32>,
    xstart: i32,
    level: u32,
    generator: &dyn terrain::Generator,
) {
    let unit_size = unit_size_for_level(level);
    for z in zrange.clone() {
        for x in xstart..xstart + CM_TEXTURE_SIZE as i32 {
            let x_pos = x as f32 * unit_size;
            let z_pos = z as f32 * unit_size;
            let x_mod = x as u32 % CM_TEXTURE_SIZE;
            let z_mod = z as u32 % CM_TEXTURE_SIZE;
            //TODO: calculate normal of point, but expensive to call 4 times noise? -> first create heights, then derive rest
            clipmap.set_height(x_mod, z_mod, level, generator.generate([x_pos, z_pos]));
        }
    }
}

fn update_heightmap(
    mut clipmap: &mut Clipmap,
    center: [f32; 2],
    level: u32,
    generator: &dyn terrain::Generator,
) {
    let base_x = snap_to_index_for_level(center[0], level) - BASE_OFFSET as i32;
    let base_z = snap_to_index_for_level(center[1], level) - BASE_OFFSET as i32;
    if let Some(previous) = clipmap.base[level as usize] {
        if !equal_coords(&previous, &[base_x, base_z]) {
            let xrows = calculate_update_range_1d(previous[0], base_x, CM_TEXTURE_SIZE as i32);
            let zrows = calculate_update_range_1d(previous[1], base_z, CM_TEXTURE_SIZE as i32);
            update_xrows(&mut clipmap, &xrows, base_z, level, &*generator);
            update_zrows(&mut clipmap, &zrows, base_x, level, &*generator);

            let xranges = calculate_copy_ranges_1d(&xrows, CM_TEXTURE_SIZE);
            for xrange in xranges {
                clipmap.copy_regions.push(CopyDescription {
                    offset: 0,
                    x: xrange.start,
                    y: 0,
                    xlen: xrange.end - xrange.start,
                    ylen: CM_TEXTURE_SIZE,
                    level,
                });
            }
            let zranges = calculate_copy_ranges_1d(&zrows, CM_TEXTURE_SIZE);
            for zrange in zranges {
                clipmap.copy_regions.push(CopyDescription {
                    offset: 0,
                    x: 0,
                    y: zrange.start,
                    xlen: CM_TEXTURE_SIZE,
                    ylen: zrange.end - zrange.start,
                    level,
                });
            }
        }
    } else {
        update_xrows(
            &mut clipmap,
            &(base_x..base_x + CM_TEXTURE_SIZE as i32),
            base_z,
            level,
            generator,
        );
        clipmap.copy_regions.push(CopyDescription {
            offset: 0,
            x: 0,
            y: 0,
            xlen: CM_TEXTURE_SIZE,
            ylen: CM_TEXTURE_SIZE,
            level,
        });
    }
    //TODO: as bytes_of_row needs to be a multiple of 256 bytes, we will figure the partial copy out after adding normals
    clipmap.base[level as usize] = Some([base_x, base_z]);
}

struct CopyDescription {
    pub offset: u32,
    pub x: u32,
    pub y: u32,
    pub xlen: u32,
    pub ylen: u32,
    pub level: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Element {
    height: f32,
}

unsafe impl bytemuck::Pod for Element {}
unsafe impl bytemuck::Zeroable for Element {}

impl Element {
    pub fn new(height: f32) -> Self {
        Self { height }
    }
}

pub struct Clipmap {
    data: Vec<Element>,
    base: Vec<Option<[i32; 2]>>,
    copy_regions: Vec<CopyDescription>,
}

impl Clipmap {
    pub fn new(levels: u32) -> Self {
        Self {
            data: vec![Element::new(0.0); (levels * CM_TEXTURE_SIZE * CM_TEXTURE_SIZE) as usize],
            base: vec![None; levels as usize],
            copy_regions: Vec::new(),
        }
    }
    pub fn set_height(&mut self, x: u32, z: u32, level: u32, height: f32) {
        self.data
            [((CM_TEXTURE_SIZE * CM_TEXTURE_SIZE * level) + x + z * CM_TEXTURE_SIZE) as usize]
            .height = height;
    }
    pub fn get_height(&self, x: u32, z: u32, level: u32) -> f32 {
        self.data[((CM_TEXTURE_SIZE * CM_TEXTURE_SIZE * level) + x + z * CM_TEXTURE_SIZE) as usize]
            .height
    }
}

fn calculate_update_range_1d(first: i32, second: i32, size: i32) -> std::ops::Range<i32> {
    if (second - first).abs() > size {
        second..second + size
    } else if first < second {
        first + size..second + size
    } else {
        second..first
    }
}

fn calculate_copy_ranges_1d(range: &std::ops::Range<i32>, size: u32) -> Vec<std::ops::Range<u32>> {
    assert!(range.start <= range.end);
    let mut ranges = Vec::new();
    if range.start != range.end {
        let start = range.start as u32 % size;
        let end = range.end as u32 % size;
        if (range.end - range.start).abs() as u32 >= size {
            ranges.push(0..size);
        } else if start < end {
            ranges.push(start..end);
        } else {
            // check for empty ranges
            if 0 != end {
                ranges.push(0..end);
            }
            if start != size {
                ranges.push(start..size);
            }
        }
    }
    ranges
}

#[test]
fn calculate_update_range_1d_test() {
    assert_eq!(calculate_update_range_1d(0, 1, 4), 4..5);
    assert_eq!(calculate_update_range_1d(0, 6, 4), 6..10);
    assert_eq!(calculate_update_range_1d(1, 0, 4), 0..1);
    assert_eq!(calculate_update_range_1d(0, -1, 4), -1..0);
    assert_eq!(calculate_update_range_1d(0, -10, 4), -10..-6);
    assert_eq!(calculate_update_range_1d(0, 3, 1), 3..4);
}

#[test]
fn calculate_copy_range_1d_test() {
    assert_eq!(calculate_copy_ranges_1d(&(3..5), 4), [0..1, 3..4]);
    assert_eq!(calculate_copy_ranges_1d(&(0..4), 4), [0..4]);
    assert_eq!(calculate_copy_ranges_1d(&(1..5), 4), [0..4]);
    assert_eq!(calculate_copy_ranges_1d(&(2..5), 4), [0..1, 2..4]);
    assert_eq!(calculate_copy_ranges_1d(&(-6..-1), 4), [0..4]);
    assert_eq!(calculate_copy_ranges_1d(&(-3..-1), 4), [1..3]);
    assert_eq!(calculate_copy_ranges_1d(&(-2..1), 4), [0..1, 2..4]);
    assert_eq!(calculate_copy_ranges_1d(&(-2..2), 4), [0..4]);
}
