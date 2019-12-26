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
