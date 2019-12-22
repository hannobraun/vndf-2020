use ggez::{
    Context,
    input::keyboard::{
        KeyCode,
        is_key_pressed,
    },
};


pub fn rotation(context: &mut Context) -> Rotation {
    let left  = is_key_pressed(context, KeyCode::Left);
    let right = is_key_pressed(context, KeyCode::Right);

    match (left, right) {
        (true, false) => Rotation::Left,
        (false, true) => Rotation::Right,
        _             => Rotation::None,
    }
}


#[derive(Clone, Copy)]
pub enum Rotation {
    Left  = -1,
    Right = 1,
    None  = 0,
}
