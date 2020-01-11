use serde::{
    Deserialize,
    Serialize,
};



#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum Event {
    Rotate(Rotation),
    Thrust(bool),
    LaunchMissile,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum Rotation {
    Left  = -1,
    Right = 1,
    None  = 0,
}
