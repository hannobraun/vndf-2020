use cgmath::prelude::*;

use crate::{
    input::Rotation,
    math::Rad,
    state::components::Body,
};


pub struct Ship {
    pub rotation: Rotation,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            rotation: Rotation::None,
        }
    }

    pub fn update(&self, body: &mut Body) {
        let rotation = self.rotation as i32 as f32;
        body.rot = Rad::full_turn() * 0.4 * rotation;
    }
}
