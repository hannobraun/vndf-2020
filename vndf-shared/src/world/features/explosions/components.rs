use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};
use toadster::Handle;

use crate::world::{
    health::Health,
    math::Scalar,
    physics::{Position, Velocity},
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Explosion {
    pub pos: Handle<Position>,
    pub vel: Handle<Velocity>,

    pub strength_total: Scalar,
    pub strength_left: Scalar,
}

impl Explosion {
    pub fn new(
        pos: impl Into<Handle<Position>>,
        vel: impl Into<Handle<Velocity>>,
        strength: Scalar,
    ) -> Self {
        Self {
            pos: pos.into(),
            vel: vel.into(),

            strength_total: strength,
            strength_left: strength,
        }
    }

    pub fn to_weak(&self) -> Self {
        Self {
            pos: self.pos.as_weak(),
            vel: self.vel.as_weak(),
            strength_total: self.strength_total.clone(),
            strength_left: self.strength_left.clone(),
        }
    }

    pub fn damage_nearby<'r, P, H>(
        &self,
        pos: &Position,
        nearby: impl IntoIterator<Item = (P, H)>,
    ) where
        P: Deref<Target = Position>,
        H: DerefMut<Target = Health>,
    {
        for (nearby_pos, mut health) in nearby {
            let distance = (nearby_pos.0 - pos.0).length();

            if distance <= 20.0 {
                health.value -= self.strength_total;
            }
        }
    }

    pub fn update(&mut self, dt: Scalar) -> bool {
        if self.strength_left > 0.0 {
            self.strength_left -= dt;
            false
        } else {
            true
        }
    }
}
