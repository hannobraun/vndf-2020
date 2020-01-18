use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    game::{
        PlayerId,
        components::Body,
        entities as e,
    },
    math::{
        prelude::*,
        Vec2,
    },
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Craft {
    pub engine_on: bool,
    pub thrust:    f32,
    pub fuel:      f32,
    pub health:    f32,
    pub owner:     PlayerId,
}

impl Craft {
    pub fn apply_thrust(&mut self, body: &mut Body, dt: f32) {
        body.acc = if self.engine_on && self.fuel > 0.0 {
            self.fuel -= self.thrust * dt;
            body.dir.normalize() * self.thrust
        }
        else {
            Vec2::zero()
        };
    }

    pub fn should_explode(&self, body: &Body) -> Option<e::Explosion> {
        if self.health <= 0.0 {
            Some(e::explosion(body))
        }
        else {
            None
        }
    }
}
