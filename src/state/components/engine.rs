use cgmath::prelude::*;

use crate::{
    math::{
        Vec2,
        rotate,
    },
    state::components::Body,
};


pub struct Engine {
    pub enabled: bool,
    pub thrust:  f32,
}

impl Engine {
    pub fn update(&self, body: &mut Body) {
        body.acc = if self.enabled {
            rotate(Vec2::unit_x(), body.dir) * self.thrust
        }
        else {
            Vec2::zero()
        };
    }
}
