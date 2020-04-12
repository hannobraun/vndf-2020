use std::{
    io::{
        self,
        Cursor,
    },
    mem::size_of,
};

use winit::event::{
    Event,
    WindowEvent,
};
use zerocopy::AsBytes as _;

use crate::graphics::{
    self,
    math::ClipUnit,
    transforms,
};

use super::{
    meshes::{
        self,
        Meshes,
        Vertex,
    },
    window::Window,
};


pub struct Renderer {
    pub surface:               wgpu::Surface,
    pub device:                wgpu::Device,
    pub queue:                 wgpu::Queue,
    pub uniform_buffer:        wgpu::Buffer,
    pub vertex_buffer:         wgpu::Buffer,
    pub index_buffer:          wgpu::Buffer,
    pub bind_group:            wgpu::BindGroup,
    pub render_pipeline:       wgpu::RenderPipeline,
    pub swap_chain_descriptor: wgpu::SwapChainDescriptor,
    pub swap_chain:            wgpu::SwapChain,

    meshes: Meshes,
}

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self, Error> {
        let surface = wgpu::Surface::create(&window.0);

        let adapter =
            wgpu::Adapter::request(
                &wgpu::RequestAdapterOptions {
                    power_preference:   wgpu::PowerPreference::Default,
                    compatible_surface: Some(&surface),
                },
                wgpu::BackendBit::all(),
            )
            .await
            .ok_or(Error::AdapterRequest)?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    extensions: wgpu::Extensions {
                        anisotropic_filtering: false,
                    },
                    limits: wgpu::Limits::default(),
                },
            )
            .await;

        let uniform_buffer = device.create_buffer_with_data(
            transforms::Transform::<ClipUnit, ClipUnit>::identity()
                .to_3d()
                .to_row_arrays()
                .as_bytes(),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );

        let meshes = Meshes::new()
            .map_err(|err| Error::Meshes(err))?;

        let vertex_buffer = device.create_buffer_with_data(
            meshes.ship.vertices.as_bytes(),
            wgpu::BufferUsage::VERTEX,
        );
        let index_buffer = device.create_buffer_with_data(
            meshes.ship.indices.as_bytes(),
            wgpu::BufferUsage::INDEX,
        );

        let vertex_shader = include_bytes!("shaders/shader.vert.spv");
        let vertex_module_module = device.create_shader_module(
            &wgpu::read_spirv(Cursor::new(&vertex_shader[..]))?,
        );

        let fragment_shader = include_bytes!("shaders/shader.frag.spv");
        let fragment_module_module = device.create_shader_module(
            &wgpu::read_spirv(Cursor::new(&fragment_shader[..]))?,
        );

        let bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                bindings: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::VERTEX,
                        ty: wgpu::BindingType::UniformBuffer { dynamic: false},
                    },
                ],
                label: None,
            },
        );
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout:   &bind_group_layout,
                bindings: &[
                    wgpu::Binding {
                        binding: 0,
                        resource: wgpu::BindingResource::Buffer {
                            buffer: &uniform_buffer,
                            range: 0 .. size_of::<Transform>() as u64,
                        },
                    }
                ],
                label: None,
            },
        );
        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
            },
        );

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                layout: &pipeline_layout,
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module:      &vertex_module_module,
                    entry_point: "main",
                },
                fragment_stage: Some(
                    wgpu::ProgrammableStageDescriptor {
                        module:      &fragment_module_module,
                        entry_point: "main",
                    }
                ),
                rasterization_state: Some(
                    wgpu::RasterizationStateDescriptor {
                        front_face:             wgpu::FrontFace::Ccw,
                        cull_mode:              wgpu::CullMode::None,
                        depth_bias:             0,
                        depth_bias_slope_scale: 0.0,
                        depth_bias_clamp:       0.0,
                    }
                ),
                primitive_topology: wgpu::PrimitiveTopology::TriangleStrip,
                color_states: &[
                    wgpu::ColorStateDescriptor {
                        format:      wgpu::TextureFormat::Bgra8UnormSrgb,
                        color_blend: wgpu::BlendDescriptor::REPLACE,
                        alpha_blend: wgpu::BlendDescriptor::REPLACE,
                        write_mask:  wgpu::ColorWrite::ALL,
                    },
                ],
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format:   wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[
                        wgpu::VertexBufferDescriptor {
                            stride: size_of::<Vertex>() as u64,
                            step_mode: wgpu::InputStepMode::Vertex,
                            attributes: &[
                                wgpu::VertexAttributeDescriptor {
                                    format: wgpu::VertexFormat::Float2,
                                    offset: 0,
                                    shader_location: 0,
                                },
                            ],
                        },
                    ],
                },
                sample_count:              1,
                sample_mask:               !0,
                alpha_to_coverage_enabled: false,
            },
        );

        let size = window.0.inner_size();

        let swap_chain_descriptor = wgpu::SwapChainDescriptor {
            usage:        wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format:       wgpu::TextureFormat::Bgra8UnormSrgb,
            width:        size.width,
            height:       size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(
            &surface,
            &swap_chain_descriptor,
        );

        Ok(
            Self {
                surface,
                device,
                queue,
                uniform_buffer,
                vertex_buffer,
                index_buffer,
                render_pipeline,
                bind_group,
                swap_chain_descriptor,
                swap_chain,

                meshes,
            }
        )
    }

    pub fn handle_event(&mut self, event: &Event<()>)
        -> Result<(), wgpu::TimeOut>
    {
        match event {
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                self.swap_chain_descriptor.width  = size.width;
                self.swap_chain_descriptor.height = size.height;

                self.swap_chain = self.device.create_swap_chain(
                    &self.surface,
                    &self.swap_chain_descriptor,
                );
            }
            Event::RedrawRequested(_) => {
                let frame = self.swap_chain.get_next_texture()?;

                let mut encoder = self.device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor { label: None }
                );

                {
                    let mut render_pass = encoder.begin_render_pass(
                        &wgpu::RenderPassDescriptor {
                            color_attachments: &[
                                wgpu::RenderPassColorAttachmentDescriptor {
                                    attachment:     &frame.view,
                                    resolve_target: None,
                                    load_op:        wgpu::LoadOp::Clear,
                                    store_op:       wgpu::StoreOp::Store,
                                    clear_color:    graphics::BACKGROUND_COLOR,
                                }
                            ],
                            depth_stencil_attachment: None,
                        },
                    );
                    render_pass.set_pipeline(&self.render_pipeline);
                    render_pass.set_bind_group(0, &self.bind_group, &[]);
                    render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
                    render_pass.set_index_buffer(&self.index_buffer, 0, 0);

                    render_pass.draw_indexed(
                        0 .. self.meshes.ship.indices.len() as u32,
                        0,
                        0 .. 1,
                    );
                }

                self.queue.submit(&[encoder.finish()]);
            }
            _ => {}
        }

        Ok(())
    }
}


pub type Transform = [[f32; 4]; 4];


#[derive(Debug)]
pub enum Error {
    AdapterRequest,
    Io(io::Error),
    Meshes(meshes::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
