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
        physics::{
            Body,
            Position,
            Velocity,
        },
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
    pub fuel:   Handle,
    pub health: Handle,

    pub engine_on: bool,
    pub thrust:    f32,
    pub owner:     PlayerId,
}

impl Craft {
    pub fn apply_thrust(&mut self,
        dt:     f32,
        bodies: &mut Store<Body>,
        fuels:  &mut Store<Fuel>,
    )
        -> Option<()>
    {
        let body = bodies.get_mut(self.body)?;
        let fuel = fuels.get_mut(self.fuel)?;

        body.acc = if self.engine_on && fuel.0 > 0.0 {
            fuel.0 -= self.thrust * dt;
            body.dir.normalize() * self.thrust
        }
        else {
            Vec2::zero()
        };

        Some(())
    }

    pub fn remove(
        handle:     Handle,
        bodies:     &mut Store<Body>,
        crafts:     &mut Store<Craft>,
        healths:    &mut Store<Health>,
        positions:  &mut Store<Position>,
        velocities: &mut Store<Velocity>,
    )
        -> Option<()>
    {
        let craft = crafts.remove(handle)?;

        Body::remove(craft.body, bodies, positions, velocities);
        healths.remove(craft.health);

        Some(())
    }
}


#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct Fuel(pub f32);
