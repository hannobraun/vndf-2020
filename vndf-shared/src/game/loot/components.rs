use serde::{
    Deserialize,
    Serialize,
};

use crate::cgs::Handle;


#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct Loot {
    pub body:     Handle,
    pub fuel:     f32,
    pub missiles: u64,
}
