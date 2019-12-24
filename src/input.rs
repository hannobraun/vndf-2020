use ggez::input::keyboard::KeyCode;


pub enum Event {
    Rotate(Rotation),
    Thrust(bool),
    LaunchMissile,
}

impl Event {
    pub fn key_down(key_code: KeyCode) -> Option<Self> {
        match key_code {
            KeyCode::Left   => Some(Self::Rotate(Rotation::Left)),
            KeyCode::Right  => Some(Self::Rotate(Rotation::Right)),
            KeyCode::Up     => Some(Self::Thrust(true)),
            KeyCode::Return => Some(Self::LaunchMissile),
            _               => None,
        }
    }

    pub fn key_up(key_code: KeyCode) -> Option<Self> {
        match key_code {
            KeyCode::Left  => Some(Self::Rotate(Rotation::None)),
            KeyCode::Right => Some(Self::Rotate(Rotation::None)),
            KeyCode::Up    => Some(Self::Thrust(false)),
            _              => None,
        }
    }
}


#[derive(Clone, Copy)]
pub enum Rotation {
    Left  = -1,
    Right = 1,
    None  = 0,
}
