use ggez::input::keyboard::KeyCode;


pub enum Event {
    Rotate(Rotation),
    Thrust(bool),
    LaunchMissile,
}

#[derive(Clone, Copy)]
pub enum Rotation {
    Left  = -1,
    Right = 1,
    None  = 0,
}


pub fn key_down(key_code: KeyCode) -> Option<Event> {
    match key_code {
        KeyCode::Left   => Some(Event::Rotate(Rotation::Left)),
        KeyCode::Right  => Some(Event::Rotate(Rotation::Right)),
        KeyCode::Up     => Some(Event::Thrust(true)),
        KeyCode::Return => Some(Event::LaunchMissile),
        _               => None,
    }
}

pub fn key_up(key_code: KeyCode) -> Option<Event> {
    match key_code {
        KeyCode::Left  => Some(Event::Rotate(Rotation::None)),
        KeyCode::Right => Some(Event::Rotate(Rotation::None)),
        KeyCode::Up    => Some(Event::Thrust(false)),
        _              => None,
    }
}
