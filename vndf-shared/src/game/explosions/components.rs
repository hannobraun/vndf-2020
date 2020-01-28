use std::ops::{
    Deref,
    DerefMut,
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    game::{
        health::Health,
        physics::Body,
    },
    math::prelude::*,
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Explosion {
    pub body: u64,

    pub strength_total: f32,
    pub strength_left:  f32,
}

impl Explosion {
    pub fn new(body: hecs::Entity, strength: f32) -> Self {
        Self {
            body: body.to_bits(),

            strength_total: strength,
            strength_left:  strength,
        }
    }

    pub fn damage_nearby<'r, B, H>(&self,
        body:   &Body,
        nearby: impl IntoIterator<Item=(B, H)>,
    )
        where
            B: Deref<Target=Body>,
            H: DerefMut<Target=Health>,
    {
        for (nearby_body, mut health) in nearby {
            let distance  = (nearby_body.pos - body.pos).magnitude();

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


pub struct Explosive {
    pub parent: Option<u64>,
}

impl Explosive {
    pub fn new() -> Self {
        Self {
            parent: None,
        }
    }
}
