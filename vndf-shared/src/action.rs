use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct Action {
    pub seq: u64,
    pub kind: Kind,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum Kind {
    Rotate(Rotation),
    Thrust(bool),
    FtlJump,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum Rotation {
    Pos = 1,
    Neg = -1,
    None = 0,
}
