use cgmath::prelude::*;

use crate::{
    input::Rotation,
    math::Rad,
    state::{
        components::Body,
        entities as e,
    },
};


pub struct Ship {
    pub rotation: Rotation,
    pub missiles: u64,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            rotation: Rotation::None,
            missiles: 16,
        }
    }

    pub fn launch_missile(&mut self, body: &Body) -> Option<e::Missile> {
        if self.missiles > 0 {
            self.missiles -= 1;
            Some(e::missile(body))
        }
        else {
            None
        }
    }

    pub fn update(&self, body: &mut Body) {
        let rotation = self.rotation as i32 as f32;
        body.rot = Rad::full_turn() * 0.4 * rotation;
    }
}
