mod window;


use std::io::{
    self,
    Cursor,
};

use winit::{
    event::{
        ElementState,
        Event,
        WindowEvent,
    },
    event_loop::{
        ControlFlow,
        EventLoop,
    },
};

use crate::{
    game::{
        Game,
        config::Key,
        coords::Screen,
        input::{
            Input,
            Transition,
        },
    },
    graphics,
    shared::math::Vec2,
};

use self::window::Window;


pub fn start(mut game: Game) -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)
        .map_err(|err| Error::Winit(err))?;

    let size    = window.0.inner_size();
    let surface = wgpu::Surface::create(&window.0);

    let adapter =
        wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                backends:         wgpu::BackendBit::all(),
            },
        )
        .ok_or(Error::AdapterRequest)?;

    let (device, mut queue) = adapter.request_device(
        &wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions { anisotropic_filtering: false },
            limits:     wgpu::Limits::default(),
        },
    );

    let vertex_shader = include_bytes!("bespoke/shaders/shader.vert.spv");
    let vertex_module_module = device.create_shader_module(
        &wgpu::read_spirv(Cursor::new(&vertex_shader[..]))?,
    );

    let fragment_shader = include_bytes!("bespoke/shaders/shader.frag.spv");
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

    let mut swap_chain_descriptor = wgpu::SwapChainDescriptor {
        usage:        wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format:       wgpu::TextureFormat::Bgra8UnormSrgb,
        width:        size.width,
        height:       size.height,
        present_mode: wgpu::PresentMode::Vsync,
    };

    let mut swap_chain = device.create_swap_chain(
        &surface,
        &swap_chain_descriptor,
    );

    let mut screen_size = Screen(
        Vec2::new(
            size.width  as f32,
            size.height as f32,
        )
    );

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared => {
                window.0.request_redraw()
            }
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                screen_size.0.x = size.width  as f32;
                screen_size.0.y = size.height as f32;

                swap_chain_descriptor.width  = size.width;
                swap_chain_descriptor.height = size.height;

                swap_chain = device.create_swap_chain(
                    &surface,
                    &swap_chain_descriptor,
                );
            }
            Event::RedrawRequested(_) => {
                let frame = swap_chain.get_next_texture();

                let mut encoder = device.create_command_encoder(
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
                    render_pass.set_pipeline(&render_pipeline);
                    render_pass.set_bind_group(0, &bind_group, &[]);
                    render_pass.draw(0 .. 0, 0 .. 0);
                }

                queue.submit(&[encoder.finish()]);
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key) = input.virtual_keycode {
                            let key = Key::Keyboard(key);

                            let input = match input.state {
                                ElementState::Pressed  => Input::KeyDown(key),
                                ElementState::Released => Input::KeyUp(key),
                            };

                            let trans = game.input.handle(
                                input,
                                &game.state.camera,
                                screen_size,
                                &mut game.events,
                            );

                            if let Transition::Quit = trans {
                                *control_flow = ControlFlow::Exit
                            }
                        }
                    },
                    _ => (),
                }
            }
            _ => {}
        }
    });
}


#[derive(Debug)]
pub enum Error {
    AdapterRequest,
    Io(io::Error),
    Winit(winit::error::OsError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
