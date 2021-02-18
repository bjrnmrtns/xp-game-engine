use crate::{
    registry::{Handle, Registry},
    renderer::{
        depth_texture::DepthTexture, error::RendererError, light_bindgroup::Transform, BindGroup,
        Light, LightBindGroup, Mesh, Renderer, Vertex,
    },
};
use nalgebra_glm::{vec3, Mat4};
use std::io::Read;

pub struct LightPipeline {
    render_pipeline: wgpu::RenderPipeline,
}

impl LightPipeline {
    pub async fn new(
        renderer: &Renderer,
        bind_group: &LightBindGroup,
    ) -> Result<Self, RendererError> {
        let (mut spirv_vs_bytes, mut spirv_fs_bytes) = (Vec::new(), Vec::new());
        match glsl_to_spirv::compile(
            include_str!("shaders/light_shader.vert"),
            glsl_to_spirv::ShaderType::Vertex,
        ) {
            Ok(mut spirv_vs_output) => {
                spirv_vs_output.read_to_end(&mut spirv_vs_bytes).unwrap();
            }
            Err(ref e) => return Err(RendererError::from(e.clone())),
        }
        match glsl_to_spirv::compile(
            include_str!("shaders/light_shader.frag"),
            glsl_to_spirv::ShaderType::Fragment,
        ) {
            Ok(mut spirv_vs_output) => {
                spirv_vs_output.read_to_end(&mut spirv_fs_bytes).unwrap();
            }
            Err(ref e) => return Err(RendererError::from(e.clone())),
        }
        let vs_module_source = wgpu::util::make_spirv(spirv_vs_bytes.as_slice());
        let fs_module_source = wgpu::util::make_spirv(spirv_fs_bytes.as_slice());
        let vs_module = renderer
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: None,
                source: vs_module_source,
                flags: Default::default(),
            });
        let fs_module = renderer
            .device
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: None,
                source: fs_module_source,
                flags: Default::default(),
            });
        let render_pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&bind_group.bind_group_layout],
                    push_constant_ranges: &[],
                });

        let render_pipeline =
            renderer
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &vs_module,
                        entry_point: "main",
                        buffers: &[Vertex::desc()],
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
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
        light_handle: &Handle<Mesh>,
        meshes: &Registry<Mesh>,
        lights: &Registry<Light>,
        light_pipeline_bindgroup: &LightBindGroup,
        renderer: &mut Renderer,
        target: &wgpu::TextureView,
    ) {
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
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &renderer.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });
            let mut transforms = Vec::new();
            for (_, light) in &lights.registry {
                match light {
                    Light::Spot(properties) => {
                        transforms.push(Transform {
                            m: nalgebra_glm::translation(&vec3(
                                properties.position[0],
                                properties.position[1],
                                properties.position[2],
                            )),
                        });
                    }
                    Light::Point(properties) => {
                        transforms.push(Transform {
                            m: nalgebra_glm::translation(&vec3(
                                properties.position[0],
                                properties.position[1],
                                properties.position[2],
                            )),
                        });
                    }
                    _ => (),
                }
            }
            light_pipeline_bindgroup.update_instance(&renderer, transforms.as_slice());
            let mesh = meshes.get(light_handle.clone()).unwrap();
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &light_pipeline_bindgroup.bind_group, &[]);
            render_pass.draw(0..mesh.len, 0..transforms.len() as u32);
        }
        renderer.queue.submit(std::iter::once(encoder.finish()));
    }
}
