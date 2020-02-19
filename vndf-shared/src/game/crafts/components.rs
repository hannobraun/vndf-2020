use serde::{
    Deserialize,
    Serialize,
};
use toadster::{
    Handle,
    Store,
    handle,
    store,
};

use crate::{
    game::{
        health::Health,
        physics::{
            Body,
            Direction,
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


#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Craft {
    pub body:   Handle<Body>,
    pub fuel:   Handle<Fuel>,
    pub health: Handle<Health>,

    pub engine_on: bool,
    pub thrust:    f32,
    pub owner:     PlayerId,
}

impl Craft {
    pub fn to_weak(&self) -> Self {
        Self {
            body:      self.body.as_weak(),
            fuel:      self.fuel.as_weak(),
            health:    self.health.as_weak(),
            engine_on: self.engine_on.clone(),
            thrust:    self.thrust.clone(),
            owner:     self.owner.clone(),
        }
    }

    pub fn apply_thrust(&mut self,
        dt:     f32,
        bodies:     &mut impl Store<Body>,
        directions: &impl Store<Direction>,
        fuels:      &mut impl Store<Fuel>,
    )
        -> Option<()>
    {
        let body = bodies.get_mut(&self.body)?;
        let dir  = directions.get(&body.dir)?;
        let fuel = fuels.get_mut(&self.fuel)?;

        body.acc = if self.engine_on && fuel.0 > 0.0 {
            fuel.0 -= self.thrust * dt;
            dir.0.normalize() * self.thrust
        }
        else {
            Vec2::zero()
        };

        Some(())
    }

    pub fn remove(
        handle:     handle::Strong<Craft>,
        bodies:     &mut store::Strong<Body>,
        crafts:     &mut store::Strong<Craft>,
        directions: &mut store::Strong<Direction>,
        fuels:      &mut store::Strong<Fuel>,
        positions:  &mut store::Strong<Position>,
        velocities: &mut store::Strong<Velocity>,
    )
        -> Option<()>
    {
        let craft = crafts.remove(handle)?;

        Body::remove(
            craft.body.strong(),
            bodies,
            directions,
            positions,
            velocities,
        );
        fuels.remove(craft.fuel.strong());

        Some(())
    }
}


#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct Fuel(pub f32);

impl Fuel {
    pub fn to_weak(&self) -> Self {
        Self(self.0.clone())
    }
}
