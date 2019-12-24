use ggez::{
    Context,
    input::keyboard::{
        KeyCode,
        is_key_pressed,
    },
};


#[derive(Clone, Copy)]
pub struct Input {
    pub rotation: Rotation,
    pub thrust:   bool,
}

impl Input {
    pub fn none() -> Self {
        Self {
            rotation: Rotation::None,
            thrust:   false,
        }
    }

    pub fn read(context: &mut Context) -> Self {
        Self {
            rotation: Rotation::read(context),
            thrust:   is_key_pressed(context, KeyCode::Up),
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
