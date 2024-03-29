use serde::{Deserialize, Serialize};
use toadster::{store, Handle};

use crate::world::{
    math::{self, rotate, Angle, Pnt2, Scalar, Vec2},
    planets::{Planet, Planets},
};

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Position(pub Pnt2);

impl Position {
    pub fn new() -> Self {
        Self(Pnt2::new(0.0, 0.0))
    }

    pub fn to_weak(&self) -> Self {
        Self(self.0.clone())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new() -> Self {
        Self(Vec2::new(0.0, 0.0))
    }

    pub fn to_weak(&self) -> Self {
        Self(self.0.clone())
    }
}

/// A physical body
///
/// Data that changes seldomly is kept in this struct itself. Data that is
/// derived from other data in `Body`, changes regularly, and thus is lends
/// itself to being interpolated client-side, is kept in separate components, so
/// it can be sent separately, at different rates.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Body {
    pub pos: Handle<Position>,
    pub vel: Handle<Velocity>,
    pub acc: Vec2,

    pub dir: Vec2,
    pub rot: Angle,

    pub mass: Scalar,
    pub time_factor: Scalar,
}

impl Body {
    pub fn new(
        pos: impl Into<Handle<Position>>,
        vel: impl Into<Handle<Velocity>>,
    ) -> Self {
        Self {
            pos: pos.into(),
            vel: vel.into(),
            acc: Vec2::zero(),

            dir: Vec2::new(1.0, 0.0),
            rot: Angle::zero(),

            mass: 1.0,
            time_factor: 1.0,
        }
    }

    pub fn to_weak(&self) -> Self {
        Self {
            pos: self.pos.as_weak(),
            vel: self.vel.as_weak(),
            acc: self.acc.clone(),
            dir: self.dir.clone(),
            rot: self.rot.clone(),
            mass: self.mass.clone(),
            time_factor: self.time_factor.clone(),
        }
    }

    pub fn update(
        &mut self,
        dt: Scalar,
        planets: &Planets<impl for<'r> store::Values<'r, Planet>>,
        mut positions: impl store::GetMut<Position>,
        mut velocities: impl store::GetMut<Velocity>,
    ) -> Option<()> {
        let vel = velocities.get_mut(&self.vel)?;
        let pos = positions.get_mut(&self.pos)?;

        let dt = dt * self.time_factor;

        self.dir = rotate(self.dir, self.rot * dt);

        math::integrate(dt, &mut pos.0, &mut vel.0, |pos| {
            self.acc + planets.acceleration_at(pos)
        });
        self.acc = Vec2::zero();

        Some(())
    }
}
