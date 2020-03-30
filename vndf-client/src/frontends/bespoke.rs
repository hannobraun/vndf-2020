mod input_handler;
mod renderer;
mod window;


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
    let mut renderer = Renderer::new(&window)
        .map_err(|err| Error::Renderer(err))?;
    let mut input_handler = InputHandler::new();

    event_loop.run(move |event, _, control_flow| {
        input_handler.handle_event(
            &event,
            &mut game,
            &window,
            control_flow,
        );

        match event {
            Event::MainEventsCleared => {
                if let Err(()) = game.handle_messages() {
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => {}
        }

        window.handle_event(&event);
        renderer.handle_event(&event);
    });
}


#[derive(Debug)]
pub enum Error {
    Renderer(renderer::Error),
    Winit(winit::error::OsError),
}
