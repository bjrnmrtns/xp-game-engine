use crate::{
    entity::Entity,
    mesh::{Mesh, Vertex},
    registry::{Handle, Registry},
    renderer::{
        bindgroup::Instance, depth_texture::DepthTexture, error::RendererError, vertex_buffer::VertexBuffer, BindGroup,
        Camera, Light, Renderer,
    },
};
use std::io::Read;

pub struct Pipeline {
    render_pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub async fn new(renderer: &Renderer, bind_group: &BindGroup) -> Result<Self, RendererError> {
        let (mut spirv_vs_bytes, mut spirv_fs_bytes) = (Vec::new(), Vec::new());
        match glsl_to_spirv::compile(include_str!("shaders/shader.vert"), glsl_to_spirv::ShaderType::Vertex) {
            Ok(mut spirv_vs_output) => {
                spirv_vs_output.read_to_end(&mut spirv_vs_bytes).unwrap();
            }
            Err(ref e) => return Err(RendererError::from(e.clone())),
        }
        match glsl_to_spirv::compile(include_str!("shaders/shader.frag"), glsl_to_spirv::ShaderType::Fragment) {
            Ok(mut spirv_vs_output) => {
                spirv_vs_output.read_to_end(&mut spirv_fs_bytes).unwrap();
            }
            Err(ref e) => return Err(RendererError::from(e.clone())),
        }
        let vs_module_source = wgpu::util::make_spirv(spirv_vs_bytes.as_slice());
        let fs_module_source = wgpu::util::make_spirv(spirv_fs_bytes.as_slice());
        let vs_module = renderer.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: vs_module_source,
            flags: Default::default(),
        });
        let fs_module = renderer.device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: fs_module_source,
            flags: Default::default(),
        });
        let render_pipeline_layout = renderer.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group.bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = renderer.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[Vertex::desc()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: Some(wgpu::IndexFormat::Uint32),
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                polygon_mode: wgpu::PolygonMode::Fill,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: DepthTexture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState {
                    front: wgpu::StencilFaceState::IGNORE,
                    back: wgpu::StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: wgpu::DepthBiasState {
                    constant: 0,
                    slope_scale: 0.0,
                    clamp: 0.0,
                },
                clamp_depth: false,
            }),
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[renderer.swap_chain_descriptor.format.into()],
            }),
        });
        Ok(Self { render_pipeline })
    }

    pub fn render(
        &self,
        entities: &Registry<Entity>,
        meshes: &mut Registry<Mesh>,
        lights: &Registry<Light>,
        bindgroup: &BindGroup,
        camera: &dyn Camera,
        renderer: &mut Renderer,
        target: &wgpu::TextureView,
    ) {
        bindgroup.update_uniforms(&renderer, &lights, camera);
        let mut instance_map = Vec::new();
        let mut start_range = 0;
        let mut transforms = Vec::new();
        for (id, mesh) in &mut meshes.registry {
            if mesh.just_loaded {
                renderer
                    .vertex_buffers
                    .insert(*id, VertexBuffer::from_mesh(&renderer, mesh));
                mesh.just_loaded = false;
            }
            transforms.extend_from_slice(
                entities
                    .registry
                    .iter()
                    .filter_map(|(_, v)| {
                        if v.mesh_handle.id == *id {
                            Some(Instance {
                                m: v.transform.to_matrix(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
                    .as_slice(),
            );
            instance_map.push((Handle::<Mesh>::new(*id), start_range..transforms.len() as u32));
            start_range = transforms.len() as u32;
        }
        bindgroup.update_instances(&renderer, transforms.as_slice());
        let mut encoder = renderer
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
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
                    stencil_ops: None,
                }),
            });

            for (mesh_handle, instance_range) in instance_map {
                if !instance_range.is_empty() {
                    let mesh = renderer.vertex_buffers.get(&mesh_handle.id).unwrap();
                    render_pass.set_pipeline(&self.render_pipeline);
                    render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                    render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                    render_pass.set_bind_group(0, &bindgroup.bind_group, &[]);
                    render_pass.draw_indexed(0..mesh.len, 0, instance_range);
                }
            }
        }
        renderer.queue.submit(std::iter::once(encoder.finish()));
    }
}
