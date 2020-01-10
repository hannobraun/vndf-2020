use crate::{
    config::{
        Config,
        Key,
    },
    shared::input::{
        Event,
        Rotation,
    },
};


pub struct Input {
    pub config: Config,
}

impl Input {
    pub fn new(config: Config) -> Self {
        Self {
            config,
        }
    }

    pub fn key_down(&self, key: Key) -> Option<Event> {
        match key {
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
