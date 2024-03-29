use crate::{
    game::{
        config::{Config, Key},
        net::input::Events,
    },
    graphics,
    shared::{
        action::{self, Rotation},
        world,
    },
};

pub struct Handler {
    pub config: Config,

    pub pointer_screen: graphics::Pnt2,
    pub pointer_world: world::Pnt2,

    scroll_acc: f32,
}

impl Handler {
    pub fn new(config: Config) -> Self {
        Self {
            config,

            pointer_screen: graphics::Pnt2::new(0.0, 0.0),
            pointer_world: world::Pnt2::new(0.0, 0.0),

            scroll_acc: 0.0,
        }
    }

    pub fn handle(&mut self, input: Input, events: &mut Events) -> Transition {
        match input {
            Input::KeyDown(key) => {
                match key {
                    k if k == self.config.input.quit => {
                        return Transition::Quit;
                    }
                    k if k == self.config.input.left => {
                        // Positive rotation, because the left button has to
                        // make sense in screen space (left-negative), while the
                        // rotation will applied in world space
                        events.push(action::Kind::Rotate(Rotation::Pos))
                    }
                    k if k == self.config.input.right => {
                        // Negative rotation, because the right button has to
                        // make sense in screen space (left-negative), while the
                        // rotation will applied in world space
                        events.push(action::Kind::Rotate(Rotation::Neg))
                    }
                    k if k == self.config.input.thrust_on => {
                        events.push(action::Kind::Thrust(true))
                    }
                    k if k == self.config.input.thrust_off => {
                        events.push(action::Kind::Thrust(false))
                    }
                    _ => (),
                }
            }
            Input::KeyUp(key) => match key {
                k if k == self.config.input.left => {
                    events.push(action::Kind::Rotate(Rotation::None))
                }
                k if k == self.config.input.right => {
                    events.push(action::Kind::Rotate(Rotation::None))
                }
                _ => (),
            },
            Input::MouseWheel(y) => {
                self.scroll_acc += y;
            }
            Input::FtlJump(time) => {
                events.push(action::Kind::FtlJump(time));
            }
        }

        Transition::None
    }

    pub fn scroll_acc(&mut self) -> f32 {
        let scroll_acc = self.scroll_acc;
        self.scroll_acc = 0.0;
        scroll_acc
    }
}

#[derive(Debug)]
pub enum Input {
    KeyDown(Key),
    KeyUp(Key),
    MouseWheel(f32),
    FtlJump(world::Scalar),
}

#[must_use]
#[derive(Eq, PartialEq)]
pub enum Transition {
    None,
    Quit,
}
