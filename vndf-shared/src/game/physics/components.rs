use serde::{
    Deserialize,
    Serialize,
};

use crate::math::{
    prelude::*,
    Pnt2,
    Rad,
    Vec2,
    rotate,
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub struct Body {
    pub pos: Pnt2,
    pub vel: Vec2,
    pub acc: Vec2,

    pub dir: Vec2,
    pub rot: Rad,
}

impl Body {
    pub fn new() -> Self {
        Self {
            pos: Pnt2::new(0.0, 0.0),
            vel: Vec2::zero(),
            acc: Vec2::zero(),

            dir: Vec2::unit_x(),
            rot: Rad::zero(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.dir = rotate(self.dir, self.rot * dt);

        self.vel += self.acc * dt;
        self.pos += self.vel * dt;
    }

    pub fn enforce_boundary(&mut self, world_size: f32) {
        let boundary = world_size / 2.0;

        if self.pos.x >= boundary && self.vel.x > 0.0 {
            self.vel.x *= -1.0;
        }
        if self.pos.x <= -boundary && self.vel.x < 0.0 {
            self.vel.x *= -1.0;
        }
        if self.pos.y >= boundary && self.vel.y > 0.0 {
            self.vel.y *= -1.0;
        }
        if self.pos.y <= -boundary && self.vel.y < 0.0 {
            self.vel.y *= -1.0;
        }
    }
}