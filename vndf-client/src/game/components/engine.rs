use cgmath::prelude::*;

use crate::{
    game::components::Body,
    math::{
        Vec2,
        rotate,
    },
};


pub struct Engine {
    pub enabled: bool,
    pub thrust:  f32,
    pub fuel:    f32,
}

impl Engine {
    pub fn update(&mut self, body: &mut Body, dt: f32) {
        body.acc = if self.enabled && self.fuel > 0.0 {
            self.fuel -= self.thrust * dt;
            rotate(Vec2::unit_x(), body.dir) * self.thrust
        }
        else {
            Vec2::zero()
        };
    }
}
