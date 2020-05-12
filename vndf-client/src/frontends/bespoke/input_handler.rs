use winit::{
    dpi::LogicalPosition,
    event::{
        ElementState,
        Event,
        KeyboardInput,
        MouseScrollDelta,
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

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        InputHandler
    }

    pub fn handle_event(&mut self,
        event:        &Event<()>,
        game:         &mut Game,
        control_flow: &mut ControlFlow,
    ) {
        let input = match event {
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

                match state {
                    ElementState::Pressed  => Input::KeyDown(key),
                    ElementState::Released => Input::KeyUp(key),
                }
            }
            Event::WindowEvent {
                event: WindowEvent::MouseWheel {
                    delta,
                    ..
                },
                ..
            } => {
                let y = match delta {
                    MouseScrollDelta::LineDelta(_, y) => {
                        *y
                    }
                    MouseScrollDelta::PixelDelta(LogicalPosition { y, ..}) => {
                        *y as f32 * 0.1
                    }
                };

                Input::MouseWheel(y)
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit;
                return;
            }
            _ => {
                return;
            }
        };

        let trans = game.handle_input(input);
        if trans == Transition::Quit {
            *control_flow = ControlFlow::Exit
        }
    }
}
