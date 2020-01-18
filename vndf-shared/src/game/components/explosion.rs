use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    game::{
        components::Body,
        features::health::Health,
    },
    math::prelude::*,
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Explosion {
    pub strength_total: f32,
    pub strength_left:  f32,
}

impl Explosion {
    pub fn new(strength: f32) -> Self {
        Self {
            strength_total: strength,
            strength_left:  strength,
        }
    }

    pub fn damage_nearby<'r>(&self,
        body:   &Body,
        nearby: impl IntoIterator<Item=(&'r Body, &'r mut Health)>,
    ) {
        for (nearby_body, health) in nearby {
            let distance  = (nearby_body.pos - body.pos).magnitude();

            if distance > 20.0 {
                continue;
            }

            let damage = f32::min(1.0 / distance, 5.0);
            health.value -= damage;
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
