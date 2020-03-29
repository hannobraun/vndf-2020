mod renderer;
mod window;


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
        input::{
            Input,
            Transition,
        },
    },
    graphics,
};

use self::{
    renderer::Renderer,
    window::Window,
};


pub fn start(mut game: Game) -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)
        .map_err(|err| Error::Winit(err))?;

    let mut renderer = Renderer::new(&window)
        .map_err(|err| Error::Renderer(err))?;

    let mut screen_size = window.size();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared => {
                window.0.request_redraw()
            }
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                screen_size.0.x = size.width  as f32;
                screen_size.0.y = size.height as f32;

                renderer.swap_chain_descriptor.width  = size.width;
                renderer.swap_chain_descriptor.height = size.height;

                renderer.swap_chain = renderer.device.create_swap_chain(
                    &renderer.surface,
                    &renderer.swap_chain_descriptor,
                );
            }
            Event::RedrawRequested(_) => {
                let frame = renderer.swap_chain.get_next_texture();

                let mut encoder = renderer.device.create_command_encoder(
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
                    render_pass.set_pipeline(&renderer.render_pipeline);
                    render_pass.set_bind_group(0, &renderer.bind_group, &[]);
                    render_pass.draw(0 .. 0, 0 .. 0);
                }

                renderer.queue.submit(&[encoder.finish()]);
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
    Renderer(renderer::Error),
    Winit(winit::error::OsError),
}
