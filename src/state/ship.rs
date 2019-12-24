use cgmath::prelude::*;

use crate::{
    input::Input,
    math::{
        Rad,
        Vec2,
        rotate,
    },
    state::Body,
};


pub struct Ship {
    pub input:  Input,
    pub thrust: bool,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            input:  Input::none(),
            thrust: false,
        }
    }

    pub fn apply_input(&self, body: &mut Body) {
        let rotation = self.input.rotation as i32 as f32;
        body.rot = Rad::full_turn() * 0.4 * rotation;

        body.acc = if self.thrust {
            rotate(Vec2::unit_x(), body.dir) * 20.0
        }
        else {
            Vec2::zero()
        };
    }
}
