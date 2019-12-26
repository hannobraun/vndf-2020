use serde::{
    Deserialize,
    Serialize,
};


#[derive(Deserialize, Serialize)]
pub enum Event {
    Rotate(Rotation),
    Thrust(bool),
    LaunchMissile,
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum Rotation {
    Left  = -1,
    Right = 1,
    None  = 0,
}
