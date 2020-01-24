pub mod components;
pub mod events;
pub mod systems;


pub use self::{
    components::*,
    events::*,
    systems::*,
};


use serde::{
    Deserialize,
    Serialize,
};


#[derive(Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
pub struct PlayerId(u64);

impl PlayerId {
    pub fn first() -> Self {
        Self(0)
    }

    pub fn increment(&mut self) -> Self {
        let current = self.0;
        self.0 += 1;
        Self(current)
    }
}
