use winit::{
    dpi::PhysicalPosition,
    event::{
        ElementState, Event, KeyboardInput, MouseScrollDelta, WindowEvent,
    },
    event_loop::ControlFlow,
};

use crate::game::{config::Key, input::Input};

pub struct InputHandler;

impl InputHandler {
    pub fn new() -> Self {
        InputHandler
    }

    pub fn handle_event(
        &mut self,
        event: &Event<()>,
        control_flow: &mut ControlFlow,
    ) -> Option<Input> {
        match event {
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
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
                    ElementState::Pressed => Some(Input::KeyDown(key)),
                    ElementState::Released => Some(Input::KeyUp(key)),
                }
            }
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                let y = match delta {
                    MouseScrollDelta::LineDelta(_, y) => *y,
                    MouseScrollDelta::PixelDelta(PhysicalPosition {
                        y,
                        ..
                    }) => *y as f32 * 0.1,
                };

                Some(Input::MouseWheel(y))
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
                None
            }
            _ => None,
        }
    }
}
