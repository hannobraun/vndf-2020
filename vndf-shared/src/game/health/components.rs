use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    cgs::Handle,
    game::{
        base::ComponentHandle,
        physics::Body,
    },
};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Health {
    pub parent: Option<ComponentHandle>,
    pub body:   Handle<Body>,
    pub value:  f32,
}

impl Health {
    pub fn new(body: Handle<Body>, value: f32) -> Self {
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
