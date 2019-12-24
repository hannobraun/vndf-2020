use ggez::{
    Context,
    input::keyboard::{
        KeyCode,
        is_key_pressed,
    },
};


pub enum Event {
    Thrust(bool),
}

impl Event {
    pub fn key_down(key_code: KeyCode) -> Option<Self> {
        match key_code {
            KeyCode::Up => Some(Self::Thrust(true)),
            _           => None,
        }
    }

    pub fn key_up(key_code: KeyCode) -> Option<Self> {
        match key_code {
            KeyCode::Up => Some(Self::Thrust(false)),
            _           => None,
        }
    }
}


#[derive(Clone, Copy)]
pub struct Input {
    pub rotation: Rotation,
}

impl Input {
    pub fn none() -> Self {
        Self {
            rotation: Rotation::None,
        }
    }

    pub fn read(context: &mut Context) -> Self {
        Self {
            rotation: Rotation::read(context),
        }
    }
}


#[derive(Clone, Copy)]
pub enum Rotation {
    Left  = -1,
    Right = 1,
    None  = 0,
}

impl Rotation {
    pub fn read(context: &mut Context) -> Self {
        let left  = is_key_pressed(context, KeyCode::Left);
        let right = is_key_pressed(context, KeyCode::Right);

        match (left, right) {
            (true, false) => Rotation::Left,
            (false, true) => Rotation::Right,
            _             => Rotation::None,
        }
    }
}
