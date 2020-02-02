use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    cgs::{
        Handle,
        Store,
    },
    game::{
        health::Health,
        physics::Body,
        players::PlayerId,
    },
    math::{
        prelude::*,
        Vec2,
    },
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Craft {
    pub body:   Handle,
    pub health: Handle,

    pub engine_on: bool,
    pub thrust:    f32,
    pub fuel:      f32,
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

    pub fn remove(
        handle:  Handle,
        bodies:  &mut Store<Body>,
        crafts:  &mut Store<Craft>,
        healths: &mut Store<Health>,
    )
        -> Option<()>
    {
        let craft = crafts.remove(handle)?;

        Body::remove(craft.body, bodies);
        healths.remove(craft.health);

        Some(())
    }
}
