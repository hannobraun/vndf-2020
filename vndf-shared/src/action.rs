use serde::{
    Deserialize,
    Serialize,
};


#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct Action {
    pub seq:  u64,
    pub kind: Kind,
}


#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum Kind {
    Rotate(Rotation),
    Thrust(bool),
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum Rotation {
    Left  = -1,
    Right = 1,
    None  = 0,
}
