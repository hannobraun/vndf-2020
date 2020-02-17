use serde::{
    Deserialize,
    Serialize,
};
use toadster::Handle;

use crate::game::{
    base::ComponentHandle,
    physics::Body,
};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Health {
    pub parent: Option<ComponentHandle>,
    pub body:   Handle<Body>,
    pub value:  f32,
}

impl Health {
    pub fn new(body: impl Into<Handle<Body>>, value: f32) -> Self {
        Health {
            parent: None,
            body:   body.into(),
            value
        }
    }

    pub fn is_dead(&self) -> bool {
        self.value <= 0.0
    }
}
