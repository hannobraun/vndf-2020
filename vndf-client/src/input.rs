use ggez::input::keyboard::KeyCode;

use crate::shared::input::{
    Event,
    Rotation,
};


pub struct Input;

impl Input {
    pub fn key_down(&self, key_code: KeyCode) -> Option<Event> {
        match key_code {
            KeyCode::Left   => Some(Event::Rotate(Rotation::Left)),
            KeyCode::Right  => Some(Event::Rotate(Rotation::Right)),
            KeyCode::Up     => Some(Event::Thrust(true)),
            KeyCode::Return => Some(Event::LaunchMissile),
            _               => None,
        }
    }

    pub fn key_up(&self, key_code: KeyCode) -> Option<Event> {
        match key_code {
            KeyCode::Left  => Some(Event::Rotate(Rotation::None)),
            KeyCode::Right => Some(Event::Rotate(Rotation::None)),
            KeyCode::Up    => Some(Event::Thrust(false)),
            _              => None,
        }
    }
}
