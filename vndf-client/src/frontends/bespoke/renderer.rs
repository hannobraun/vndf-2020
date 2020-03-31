use std::io::{
    self,
    Cursor,
};

use winit::event::{
    Event,
    WindowEvent,
};

use crate::graphics;

use super::window::Window;


pub struct Renderer {
    pub surface:               wgpu::Surface,
    pub device:                wgpu::Device,
    pub queue:                 wgpu::Queue,
    pub bind_group:            wgpu::BindGroup,
    pub render_pipeline:       wgpu::RenderPipeline,
    pub swap_chain_descriptor: wgpu::SwapChainDescriptor,
    pub swap_chain:            wgpu::SwapChain,
}

impl Renderer {
    pub fn new(window: &Window) -> Result<Self, Error> {
        let surface = wgpu::Surface::create(&window.0);

        let adapter =
            wgpu::Adapter::request(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::Default,
                    backends:         wgpu::BackendBit::all(),
                },
            )
            .ok_or(Error::AdapterRequest)?;

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions { anisotropic_filtering: false },
                limits:     wgpu::Limits::default(),
            },
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
            &wgpu::BindGroupLayoutDescriptor { bindings: &[] },
        );
        let bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout:   &bind_group_layout,
                bindings: &[],
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

                depth_stencil_state:       None,
                index_format:              wgpu::IndexFormat::Uint16,
                vertex_buffers:            &[],
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
            present_mode: wgpu::PresentMode::Vsync,
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
                render_pipeline,
                bind_group,
                swap_chain_descriptor,
                swap_chain,
            }
        )
    }

    pub fn handle_event(&mut self, event: &Event<()>) {
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
                let frame = self.swap_chain.get_next_texture();

                let mut encoder = self.device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor { todo: 0 }
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
                    render_pass.draw(0 .. 0, 0 .. 0);
                }

                self.queue.submit(&[encoder.finish()]);
            }
            _ => {}
        }
    }
}


#[derive(Debug)]
pub enum Error {
    AdapterRequest,
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
