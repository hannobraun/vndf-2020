use serde::{
    Deserialize,
    Serialize,
};
use toadster::{
    Handle,
    store,
};

use crate::{
    game::planets::{
        Planet,
        Planets,
    },
    math::{
        prelude::*,
        self,
        Pnt2,
        Rad,
        Vec2,
        rotate,
    },
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
    pub rot: Rad,
}

impl Body {
    pub fn new(
        pos: impl Into<Handle<Position>>,
        vel: impl Into<Handle<Velocity>>,
    )
        -> Self
    {
        Self {
            pos: pos.into(),
            vel: vel.into(),
            acc: Vec2::zero(),

            dir: Vec2::unit_x(),
            rot: Rad::zero(),
        }
    }

    pub fn to_weak(&self) -> Self {
        Self {
            pos: self.pos.as_weak(),
            vel: self.vel.as_weak(),
            acc: self.acc.clone(),
            dir: self.dir.clone(),
            rot: self.rot.clone(),
        }
    }

    pub fn update(&mut self,
            dt:         f32,
            planets:    Planets<impl for<'r> store::Values<'r, Planet>>,
        mut positions:  impl store::GetMut<Position>,
        mut velocities: impl store::GetMut<Velocity>,
    )
        -> Option<()>
    {
        let vel = velocities.get_mut(&self.vel)?;
        let pos = positions.get_mut(&self.pos)?;

        self.dir = rotate(self.dir, self.rot * dt);

        math::integrate(
            dt,
            &mut pos.0,
            &mut vel.0,
            |pos| self.acc + planets.gravitation_at(pos),
        );
        self.acc = Vec2::zero();

        Some(())
    }

    pub fn enforce_boundary(&mut self,
        world_size: f32,
        positions:  &impl store::Get<Position>,
        velocities: &mut impl store::GetMut<Velocity>,
    )
        -> Option<()>
    {
        let boundary = world_size / 2.0;

        let pos = positions.get(&self.pos)?;
        let vel = velocities.get_mut(&self.vel)?;

        if pos.0.x >= boundary && vel.0.x > 0.0 {
            vel.0.x *= -1.0;
        }
        if pos.0.x <= -boundary && vel.0.x < 0.0 {
            vel.0.x *= -1.0;
        }
        if pos.0.y >= boundary && vel.0.y > 0.0 {
            vel.0.y *= -1.0;
        }
        if pos.0.y <= -boundary && vel.0.y < 0.0 {
            vel.0.y *= -1.0;
        }

        Some(())
    }
}
