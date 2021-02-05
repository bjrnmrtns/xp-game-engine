use crate::{
    assets::Assets,
    entity::Entity,
    renderer::{
        depth_texture::DepthTexture,
        error::RendererError,
        light::{MAX_NR_OF_DIRECTIONAL_LIGHTS, MAX_NR_OF_POINT_LIGHTS, MAX_NR_OF_SPOT_LIGHTS},
        DirectionalProperties, Light, Mesh, PointProperties, Renderer, SpotProperties, Vertex,
    },
};
use nalgebra_glm::Mat4;
use std::io::Read;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Uniforms {
    pub m: Mat4,
    pub v: Mat4,
    pub p: Mat4,
    pub world_camera_position: [f32; 3],

    pub material_ambient: [f32; 3],
    pub material_diffuse: [f32; 3],
    pub material_specular: [f32; 3],
    pub material_shininess: f32,
}

unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}

pub struct Pipeline {
    uniform_buffer: wgpu::Buffer,
    directional_light_buffer: wgpu::Buffer,
    spot_light_buffer: wgpu::Buffer,
    point_light_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub async fn new(renderer: &Renderer) -> Result<Self, RendererError> {
        let (mut spirv_vs_bytes, mut spirv_fs_bytes) = (Vec::new(), Vec::new());
        match glsl_to_spirv::compile(
            include_str!("shaders/shader.vert"),
            glsl_to_spirv::ShaderType::Vertex,
        ) {
            Ok(mut spirv_vs_output) => {
                spirv_vs_output.read_to_end(&mut spirv_vs_bytes).unwrap();
            }
            Err(ref e) => return Err(RendererError::from(e.clone())),
        }
        match glsl_to_spirv::compile(
            include_str!("shaders/shader.frag"),
            glsl_to_spirv::ShaderType::Fragment,
        ) {
            Ok(mut spirv_vs_output) => {
                spirv_vs_output.read_to_end(&mut spirv_fs_bytes).unwrap();
            }
            Err(ref e) => return Err(RendererError::from(e.clone())),
        }
        let vs_module_source = wgpu::util::make_spirv(spirv_vs_bytes.as_slice());
        let fs_module_source = wgpu::util::make_spirv(spirv_fs_bytes.as_slice());
        let vs_module = renderer.device.create_shader_module(vs_module_source);
        let fs_module = renderer.device.create_shader_module(fs_module_source);

        let uniform_buffer = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            size: (std::mem::size_of::<Uniforms>()) as u64,
            mapped_at_creation: false,
        });

        let directional_light_buffer = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            size: (std::mem::size_of::<DirectionalProperties>() * MAX_NR_OF_DIRECTIONAL_LIGHTS)
                as u64,
            mapped_at_creation: false,
        });
        let spot_light_buffer = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            size: (std::mem::size_of::<SpotProperties>() * MAX_NR_OF_SPOT_LIGHTS) as u64,
            mapped_at_creation: false,
        });

        let point_light_buffer = renderer.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            size: (std::mem::size_of::<PointProperties>() * MAX_NR_OF_POINT_LIGHTS) as u64,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
                            visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::UniformBuffer {
                                dynamic: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::UniformBuffer {
                                dynamic: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 3,
                            visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                            ty: wgpu::BindingType::UniformBuffer {
                                dynamic: false,
                                min_binding_size: None,
                            },
                            count: None,
                        },
                    ],
                    label: None,
                });

        let uniform_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &uniform_bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Buffer(directional_light_buffer.slice(..)),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: wgpu::BindingResource::Buffer(spot_light_buffer.slice(..)),
                    },
                    wgpu::BindGroupEntry {
                        binding: 3,
                        resource: wgpu::BindingResource::Buffer(point_light_buffer.slice(..)),
                    },
                ],
            });

        let render_pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&uniform_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            renderer
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
                        cull_mode: wgpu::CullMode::None,
                        clamp_depth: false,
                        depth_bias: 0,
                        depth_bias_slope_scale: 0.0,
                        depth_bias_clamp: 0.0,
                    }),
                    color_states: &[wgpu::ColorStateDescriptor {
                        format: renderer.swap_chain_descriptor.format,
                        color_blend: wgpu::BlendDescriptor::REPLACE,
                        alpha_blend: wgpu::BlendDescriptor::REPLACE,
                        write_mask: wgpu::ColorWrite::ALL,
                    }],
                    primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                    depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                        format: DepthTexture::DEPTH_FORMAT,
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
            uniform_buffer,
            directional_light_buffer,
            spot_light_buffer,
            point_light_buffer,
            uniform_bind_group,
            render_pipeline,
        })
    }

    pub fn render(
        &self,
        entity: &Entity,
        meshes: &Assets<Mesh>,
        lights: &Assets<Light>,
        projection: Mat4,
        view: Mat4,
        world_camera_position: [f32; 3],
        renderer: &mut Renderer,
    ) {
        let target = &renderer
            .swap_chain
            .get_current_frame()
            .expect("Could not get next frame texture_view")
            .output
            .view;
        let mut encoder = renderer
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &target,
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
                    attachment: &renderer.depth_texture.view,
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
            let uniforms = Uniforms {
                m: entity.model.clone(),
                v: view,
                p: projection,
                world_camera_position,
                material_ambient: [1.0, 0.5, 0.31],
                material_diffuse: [1.0, 0.5, 0.31],
                material_specular: [0.5, 0.5, 0.5],
                material_shininess: 16.0,
            };
            let mut directional_lights = Vec::new();
            let mut spot_lights = Vec::new();
            let mut point_lights = Vec::new();
            for (_, light) in &lights.assets {
                match light {
                    Light::Directional(properties) => {
                        directional_lights.push(*properties);
                    }
                    Light::Spot(properties) => {
                        spot_lights.push(*properties);
                    }
                    Light::Point(properties) => {
                        point_lights.push(*properties);
                    }
                }
            }
            assert!(directional_lights.len() <= MAX_NR_OF_DIRECTIONAL_LIGHTS);
            assert!(spot_lights.len() <= MAX_NR_OF_SPOT_LIGHTS);
            assert!(point_lights.len() <= MAX_NR_OF_POINT_LIGHTS);
            renderer
                .queue
                .write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));
            renderer.queue.write_buffer(
                &self.directional_light_buffer,
                0,
                bytemuck::cast_slice(directional_lights.as_slice()),
            );
            renderer.queue.write_buffer(
                &self.spot_light_buffer,
                0,
                bytemuck::cast_slice(spot_lights.as_slice()),
            );
            renderer.queue.write_buffer(
                &self.point_light_buffer,
                0,
                bytemuck::cast_slice(point_lights.as_slice()),
            );

            let mesh = meshes.get(entity.mesh_handle.clone()).unwrap();
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw(0..mesh.len, 0..1);
        }
        renderer.queue.submit(std::iter::once(encoder.finish()));
    }
}
