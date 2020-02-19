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

use crate::math::{
    prelude::*,
    Pnt2,
    Rad,
    Vec2,
    rotate,
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

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Direction(pub Vec2);

impl Direction {
    pub fn new() -> Self {
        Self(Vec2::unit_x())
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

    pub dir: Handle<Direction>,
    pub rot: Rad,
}

impl Body {
    pub fn new(
        pos: impl Into<Handle<Position>>,
        vel: impl Into<Handle<Velocity>>,
        dir: impl Into<Handle<Direction>>,
    )
        -> Self
    {
        Self {
            pos: pos.into(),
            vel: vel.into(),
            acc: Vec2::zero(),

            dir: dir.into(),
            rot: Rad::zero(),
        }
    }

    pub fn to_weak(&self) -> Self {
        Self {
            pos: self.pos.as_weak(),
            vel: self.vel.as_weak(),
            acc: self.acc.clone(),
            dir: self.dir.as_weak(),
            rot: self.rot.clone(),
        }
    }

    pub fn update(&mut self,
        dt:         f32,
        directions: &mut impl Store<Direction>,
        positions:  &mut impl Store<Position>,
        velocities: &mut impl Store<Velocity>,
    )
        -> Option<()>
    {
        let vel = velocities.get_mut(&self.vel)?;
        let pos = positions.get_mut(&self.pos)?;
        let dir = directions.get_mut(&self.dir)?;

        dir.0 = rotate(dir.0, self.rot * dt);

        vel.0 += self.acc * dt;
        pos.0 += vel.0 * dt;

        Some(())
    }

    pub fn enforce_boundary(&mut self,
        world_size: f32,
        positions:  &impl Store<Position>,
        velocities: &mut impl Store<Velocity>,
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

    pub fn remove(
        handle:     handle::Strong<Body>,
        bodies:     &mut store::Strong<Body>,
        directions: &mut store::Strong<Direction>,
        positions:  &mut store::Strong<Position>,
        velocities: &mut store::Strong<Velocity>,
    )
        -> Option<()>
    {
        let body = bodies.remove(handle)?;

        directions.remove(body.dir.strong());
        positions.remove(body.pos.strong());
        velocities.remove(body.vel.strong());

        Some(())
    }
}
