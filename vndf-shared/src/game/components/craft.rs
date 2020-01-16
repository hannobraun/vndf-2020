use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    game::components::Body,
    math::{
        prelude::*,
        Vec2,
    },
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Craft {
    pub enabled: bool,
    pub thrust:  f32,
    pub fuel:    f32,
}

impl Craft {
    pub fn update(&mut self, body: &mut Body, dt: f32) {
        body.acc = if self.enabled && self.fuel > 0.0 {
            self.fuel -= self.thrust * dt;
            body.dir.normalize() * self.thrust
        }
        else {
            Vec2::zero()
        };
    }
}
