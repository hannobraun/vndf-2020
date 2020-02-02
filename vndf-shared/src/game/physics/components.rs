use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    cgs::{
        Handle,
        Store,
    },
    math::{
        prelude::*,
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
}


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new() -> Self {
        Self(Vec2::new(0.0, 0.0))
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Body {
    pub pos: Handle,
    pub vel: Handle,
    pub acc: Vec2,

    pub dir: Vec2,
    pub rot: Rad,
}

impl Body {
    pub fn new(pos: Handle, vel: Handle) -> Self {
        Self {
            pos,
            vel,
            acc: Vec2::zero(),

            dir: Vec2::unit_x(),
            rot: Rad::zero(),
        }
    }

    pub fn update(&mut self,
        dt:         f32,
        positions:  &mut Store<Position>,
        velocities: &mut Store<Velocity>,
    )
        -> Option<()>
    {
        self.dir = rotate(self.dir, self.rot * dt);

        let vel = velocities.get_mut(self.vel)?;
        vel.0 += self.acc * dt;

        let pos = positions.get_mut(self.pos)?;
        pos.0 += vel.0 * dt;

        Some(())
    }

    pub fn enforce_boundary(&mut self,
        world_size: f32,
        positions:  &Store<Position>,
        velocities: &mut Store<Velocity>,
    )
        -> Option<()>
    {
        let boundary = world_size / 2.0;

        let pos = positions.get(self.pos)?;
        let vel = velocities.get_mut(self.vel)?;

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
        handle:    Handle,
        bodies:    &mut Store<Body>,
        positions: &mut Store<Position>,
    )
        -> Option<()>
    {
        let body = bodies.remove(handle)?;
        positions.remove(body.pos);
        Some(())
    }
}
