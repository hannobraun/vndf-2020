use winit::{
    event::{
        ElementState,
        Event,
        KeyboardInput,
        WindowEvent,
    },
    event_loop::ControlFlow,
};

use crate::game::{
    Game,
    config::Key,
    input::{
        Input,
        Transition,
    },
};

use super::window::Window;

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        InputHandler
    }

    pub fn handle_event(&mut self,
        event:        &Event<()>,
        game:         &mut Game,
        window:       &Window,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        state,
                        virtual_keycode: Some(key_code),
                        ..
                    },
                    ..
                },
                ..
            } => {
                let key = Key::Keyboard(*key_code);

                let input = match state {
                    ElementState::Pressed  => Input::KeyDown(key),
                    ElementState::Released => Input::KeyUp(key),
                };

                let trans = game.handle_input(input, window.size());
                if trans == Transition::Quit {
                    *control_flow = ControlFlow::Exit
                }
            }
            _ => {}
        }
    }
}
