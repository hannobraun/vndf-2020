use ggez::Context;

use crate::{
    config::{
        Config,
        Key,
    },
    shared::{
        input::{
            Event,
            Rotation,
        },
        math::Pnt2,
    },
    transforms,
};


pub struct Input {
    pub config: Config,

    pub pointer_screen: Pnt2,
    pub pointer_world:  Option<Pnt2>,
}

impl Input {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            pointer_screen: Pnt2::new(0.0, 0.0),
            pointer_world:  None,
        }
    }

    pub fn mouse_motion(&mut self, context: &mut Context, x: f32, y: f32) {
        self.pointer_screen.x = x;
        self.pointer_screen.y = y;

        self.pointer_world = transforms::screen_to_world(
            context,
            self.pointer_screen,
        );
    }

    pub fn key_down(&mut self, key: Key) -> Option<Event> {
        match key {
            k if k == self.config.input.left =>
                Some(Event::Rotate(Rotation::Left)),
            k if k == self.config.input.right =>
                Some(Event::Rotate(Rotation::Right)),
            k if k == self.config.input.thrust =>
                Some(Event::Thrust(true)),
            k if k == self.config.input.launch =>
                self.pointer_world
                    .map(|target| Event::LaunchMissile { target }),

            _ => None,
        }
    }

    pub fn key_up(&mut self, key: Key) -> Option<Event> {
        match key {
            k if k == self.config.input.left =>
                Some(Event::Rotate(Rotation::None)),
            k if k == self.config.input.right =>
                Some(Event::Rotate(Rotation::None)),
            k if k == self.config.input.thrust =>
                Some(Event::Thrust(false)),

            _ => None,
        }
    }
}
