use ggez::input::keyboard::KeyCode;

use crate::{
    config::Config,
    shared::input::{
        Event,
        Rotation,
    },
};


pub struct Input {
    config: Config,
}

impl Input {
    pub fn new(config: Config) -> Self {
        Self {
            config,
        }
    }

    pub fn key_down(&self, key_code: KeyCode) -> Option<Event> {
        match key_code {
            k if k == self.config.left =>
                Some(Event::Rotate(Rotation::Left)),
            k if k == self.config.right =>
                Some(Event::Rotate(Rotation::Right)),
            k if k == self.config.thrust =>
                Some(Event::Thrust(true)),
            k if k == self.config.launch =>
                Some(Event::LaunchMissile),

            _ => None,
        }
    }

    pub fn key_up(&self, key_code: KeyCode) -> Option<Event> {
        match key_code {
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
