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
}

impl Engine {
    pub fn update(&self, body: &mut Body) {
        body.acc = if self.enabled {
            rotate(Vec2::unit_x(), body.dir) * 20.0
        }
        else {
            Vec2::zero()
        };
    }
}
