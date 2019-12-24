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


pub struct Player {
    pub input: Input,
}

impl Player {
    pub fn new() -> Self {
        Self {
            input: Input::none(),
        }
    }

    pub fn apply_input(&self, body: &mut Body) {
        let rotation = self.input.rotation as i32 as f32;
        body.rot = Rad::turn_div_2() * rotation;

        body.acc = if self.input.thrust {
            rotate(Vec2::unit_x(), body.dir) * 300.0
        }
        else {
            Vec2::zero()
        };
    }
}
