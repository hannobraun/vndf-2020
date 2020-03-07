use serde::{
    Deserialize,
    Serialize,
};
use toadster::{
    Handle,
    Store,
};

use crate::{
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
        bodies: &mut impl Store<Body>,
        fuels:  &mut impl Store<Fuel>,
    )
        -> Option<()>
    {
        let body = bodies.get_mut(&self.body)?;
        let fuel = fuels.get_mut(&self.fuel)?;

        body.acc += if self.engine_on && fuel.0 > 0.0 {
            let max_fuel_used = self.thrust * dt;
            let fuel_used     = f32::min(max_fuel_used, fuel.0);

            fuel.0 -= fuel_used;
            body.dir.normalize() * self.thrust * fuel_used / max_fuel_used
        }
        else {
            Vec2::zero()
        };

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
