use cgmath::prelude::*;

use crate::{
    input::Rotation,
    math::{
        Rad,
        Vec2,
        rotate,
    },
    state::{
        Body,
        Missile,
    },
};


pub struct Ship {
    pub rotation: Rotation,
    pub thrust:   bool,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            rotation: Rotation::None,
            thrust:   false,
        }
    }

    pub fn launch_missile(&self, body: &Body) -> (Body, Missile) {
        (*body, Missile::new())
    }

    pub fn apply_input(&self, body: &mut Body) {
        let rotation = self.rotation as i32 as f32;
        body.rot = Rad::full_turn() * 0.4 * rotation;

        body.acc = if self.thrust {
            rotate(Vec2::unit_x(), body.dir) * 20.0
        }
        else {
            Vec2::zero()
        };
    }
}
