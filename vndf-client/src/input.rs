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
    pub config:  Config,
    pub pointer: Pnt2,
}

impl Input {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            pointer: Pnt2::new(0.0, 0.0),
        }
    }

    pub fn mouse_motion(&mut self, x: f32, y: f32) {
        self.pointer.x = x;
        self.pointer.y = y;
    }

    pub fn key_down(&self, context: &mut Context, key: Key) -> Option<Event> {
        match key {
            k if k == self.config.left =>
                Some(Event::Rotate(Rotation::Left)),
            k if k == self.config.right =>
                Some(Event::Rotate(Rotation::Right)),
            k if k == self.config.thrust =>
                Some(Event::Thrust(true)),
            k if k == self.config.launch =>
                transforms::screen_to_world(context, self.pointer)
                    .map(|target| Event::LaunchMissile { target }),

            _ => None,
        }
    }

    pub fn key_up(&self, key: Key) -> Option<Event> {
        match key {
            k if k == self.config.left =>
                Some(Event::Rotate(Rotation::None)),
            k if k == self.config.right =>
                Some(Event::Rotate(Rotation::None)),
            k if k == self.config.thrust =>
                Some(Event::Thrust(false)),

            _ => None,
        }
    }
}
