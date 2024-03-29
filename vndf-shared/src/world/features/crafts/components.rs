use serde::{Deserialize, Serialize};
use toadster::{store, Handle};

use crate::world::{
    health::Health,
    math::{Scalar, Vec2},
    physics::Body,
    players::PlayerId,
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Craft {
    pub body: Handle<Body>,
    pub fuel: Handle<Fuel>,
    pub health: Handle<Health>,

    pub engine_on: bool,
    pub thrust: Scalar,
    pub owner: PlayerId,
}

impl Craft {
    pub fn to_weak(&self) -> Self {
        Self {
            body: self.body.as_weak(),
            fuel: self.fuel.as_weak(),
            health: self.health.as_weak(),
            engine_on: self.engine_on.clone(),
            thrust: self.thrust.clone(),
            owner: self.owner.clone(),
        }
    }

    pub fn apply_thrust(
        &mut self,
        dt: Scalar,
        bodies: &mut impl store::GetMut<Body>,
        fuels: &mut impl store::GetMut<Fuel>,
    ) -> Option<()> {
        let body = bodies.get_mut(&self.body)?;
        let fuel = fuels.get_mut(&self.fuel)?;

        let force = if self.engine_on && fuel.0 > 0.0 {
            let max_fuel_used = self.thrust * dt;
            let fuel_used = Scalar::min(max_fuel_used, fuel.0);

            fuel.0 -= fuel_used;
            body.dir.normalize() * self.thrust * fuel_used / max_fuel_used
        } else {
            Vec2::zero()
        };

        body.acc += force / body.mass;

        Some(())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub struct Fuel(pub Scalar);

impl Fuel {
    pub fn to_weak(&self) -> Self {
        Self(self.0.clone())
    }
}
