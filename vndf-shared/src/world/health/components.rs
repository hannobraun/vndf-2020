use std::collections::HashSet;

use serde::{
    Deserialize,
    Serialize,
};
use toadster::{
    Handle,
    handle::{
        self,
        Untyped,
    },
};

use crate::{
    data,
    world::physics::Body,
};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Health {
    parent: Option<data::client::Handle>,

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

    pub fn finalize(&mut self,
        parent:   data::client::Handle,
        entities: &mut HashSet<handle::Strong<Untyped>>,
    ) {
        self.parent = Some(parent.as_weak());
        entities.insert(parent.into_strong_untyped());
    }

    pub fn to_weak(&self) -> Self {
        Self {
            parent: self.parent.clone(),
            body:   self.body.as_weak(),
            value:  self.value.clone(),
        }
    }

    pub fn parent(self) -> Option<data::client::Handle> {
        self.parent
    }

    pub fn parent_ref(&self) -> Option<&data::client::Handle> {
        self.parent.as_ref()
    }

    pub fn is_dead(&self) -> bool {
        self.value <= 0.0
    }
}
