use serde::{
    Deserialize,
    Serialize,
};


#[derive(Debug, Deserialize, Serialize)]
pub enum Event {
    Rotate(Rotation),
    Thrust(bool),
    LaunchMissile,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Rotation {
    Left  = -1,
    Right = 1,
    None  = 0,
}
