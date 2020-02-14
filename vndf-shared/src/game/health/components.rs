use serde::{
    Deserialize,
    Serialize,
};
use toadster::handle;

use crate::game::{
    base::ComponentHandle,
    physics::Body,
};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Health {
    pub parent: Option<ComponentHandle>,
    pub body:   handle::Strong<Body>,
    pub value:  f32,
}

impl Health {
    pub fn new(body: handle::Strong<Body>, value: f32) -> Self {
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
