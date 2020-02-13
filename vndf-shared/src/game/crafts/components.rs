use serde::{
    Deserialize,
    Serialize,
};
use toadster::{
    Store,
    StrongHandle,
    StrongStore,
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


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Craft {
    pub body:   StrongHandle<Body>,
    pub fuel:   StrongHandle<Fuel>,
    pub health: StrongHandle<Health>,

    pub engine_on: bool,
    pub thrust:    f32,
    pub owner:     PlayerId,
}

impl Craft {
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
        handle:     StrongHandle<Craft>,
        bodies:     &mut StrongStore<Body>,
        crafts:     &mut StrongStore<Craft>,
        directions: &mut StrongStore<Direction>,
        fuels:      &mut StrongStore<Fuel>,
        positions:  &mut StrongStore<Position>,
        velocities: &mut StrongStore<Velocity>,
    )
        -> Option<()>
    {
        let craft = crafts.remove(handle)?;

        Body::remove(
            craft.body,
            bodies,
            directions,
            positions,
            velocities,
        );
        fuels.remove(craft.fuel);

        Some(())
    }
}


#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct Fuel(pub f32);
