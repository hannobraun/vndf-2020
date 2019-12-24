use cgmath::prelude::*;

use crate::{
    math::{
        Pnt2,
        Rad,
        Vec2,
    },
    state::WORLD_SIZE,
};


pub struct Body {
    pub pos: Pnt2,
    pub vel: Vec2,
    pub acc: Vec2,

    pub dir: Rad,
    pub rot: Rad,
}

impl Body {
    pub fn new() -> Self {
        Self {
            pos: Pnt2::new(0.0, 0.0),
            vel: Vec2::zero(),
            acc: Vec2::zero(),
            dir: Rad::zero(),
            rot: Rad::zero(),
        }
    }

    pub fn update(&mut self, frame_time: f32) {
        self.dir += self.rot * frame_time;

        self.vel += self.acc * frame_time;
        self.pos += self.vel * frame_time;
    }

    pub fn enforce_boundary(&mut self) {
        let boundary = WORLD_SIZE / 2.0;

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
