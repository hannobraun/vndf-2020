use ggez::{
    Context,
    input::keyboard::{
        KeyCode,
        is_key_pressed,
    },
};


pub struct Input {
    pub rotation: Rotation,
}

impl Input {
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
