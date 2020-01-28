use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    cgs::Handle,
    game::base::ComponentHandle,
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Health {
    pub parent: Option<ComponentHandle>,
    pub body:   Handle,
    pub value:  f32,
}

impl Health {
    pub fn new(body: Handle, value: f32) -> Self {
        Health {
            parent: None,
            body,
            value
        }
    }

    pub fn is_dead(&self) -> bool {
        self.value <= 0.0
    }
}
