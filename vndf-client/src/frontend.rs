mod drawables;
mod drawers;
mod input_handler;
mod meshes;
mod renderer;
mod shaders;
mod ui;
mod uniforms;
mod window;


use futures::executor::block_on;
use log::error;
use time::Instant;
use winit::{
    event::{
        Event,
        WindowEvent,
    },
    event_loop::{
        ControlFlow,
        EventLoop,
    },
};

use crate::{
    Graphics,
    game::Game,
};

use self::{
    input_handler::InputHandler,
    renderer::Renderer,
    ui::Ui,
    window::Window,
};


pub fn start(mut game: Game, graphics: Graphics)
    -> Result<(), Error>
{
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)
        .map_err(|err| Error::Winit(err))?;
    let mut renderer = block_on(Renderer::new(&window, graphics))
        .map_err(|err| Error::Renderer(err))?;
    let mut input_handler = InputHandler::new();
    let mut ui = Ui::new();

    let mut time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        input_handler.handle_event(
            &event,
            &mut game,
            control_flow,
        );

        match event {
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                renderer.handle_resize(size);
            }
            Event::MainEventsCleared => {
                let dt = time.elapsed();
                time = Instant::now();

                if let Err(()) = game.update(dt) {
                    *control_flow = ControlFlow::Exit;
                }

                window.inner().request_redraw();
            }
            _ => {}
        }

        if let Err(err) = renderer.handle_event(&event, &game, &mut ui) {
            error!("Renderer error: {:?}", err);
            *control_flow = ControlFlow::Exit;
        }
    });
}


#[derive(Debug)]
pub enum Error {
    Renderer(renderer::Error),
    Winit(winit::error::OsError),
}
