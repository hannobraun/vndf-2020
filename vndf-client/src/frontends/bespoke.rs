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

use crate::game::{
    Game,
    config::Key,
    input::{
        Input,
        Transition,
    },
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

    event_loop.run(move |event, _, control_flow| {
        window.handle_event(&event);
        renderer.handle_event(&event);

        match event {
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
                                window.size(),
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
