mod drawable;
mod input_handler;
mod meshes;
mod renderer;
mod window;


use futures::executor::block_on;
use log::error;
use time::Instant;
use winit::{
    event::Event,
    event_loop::{
        ControlFlow,
        EventLoop,
    },
};

use crate::game::Game;

use self::{
    input_handler::InputHandler,
    renderer::Renderer,
    window::Window,
};


pub fn start(mut game: Game) -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)
        .map_err(|err| Error::Winit(err))?;
    let mut renderer = block_on(Renderer::new(&window))
        .map_err(|err| Error::Renderer(err))?;
    let mut input_handler = InputHandler::new();

    let mut time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        input_handler.handle_event(
            &event,
            &mut game,
            &window,
            control_flow,
        );

        match event {
            Event::MainEventsCleared => {
                let dt = time.elapsed();
                time = Instant::now();

                if let Err(()) = game.update(dt) {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }

        window.handle_event(&event);
        if let Err(err) = renderer.handle_event(&event) {
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
