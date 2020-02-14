use std::ops::{
    Deref,
    DerefMut,
};

use serde::{
    Deserialize,
    Serialize,
};
use toadster::handle;

use crate::{
    game::{
        health::Health,
        physics::{
            Body,
            Position,
        },
    },
    math::prelude::*,
};


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Explosion {
    pub body: handle::Strong<Body>,

    pub strength_total: f32,
    pub strength_left:  f32,
}

impl Explosion {
    pub fn new(body: handle::Strong<Body>, strength: f32) -> Self {
        Self {
            body,

            strength_total: strength,
            strength_left:  strength,
        }
    }

    pub fn damage_nearby<'r, P, H>(&self,
        pos:    &Position,
        nearby: impl IntoIterator<Item=(P, H)>,
    )
        where
            P: Deref<Target=Position>,
            H: DerefMut<Target=Health>,
    {
        for (nearby_pos, mut health) in nearby {
            let distance = (nearby_pos.0 - pos.0).magnitude();

            if distance <= 20.0 {
                health.value -= self.strength_total;
            }
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        if self.strength_left > 0.0 {
            self.strength_left -= dt;
            false
        }
        else {
            true
        }
    }
}
