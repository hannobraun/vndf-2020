use cgmath::prelude::*;

use crate::{
    input::Rotation,
    math::Rad,
    state::{
        Body,
        Missile,
    },
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

    pub fn launch_missile(&self, body: &Body) -> (Missile, Body) {
        let body = Body {
            rot: Rad::zero(),
            .. *body
        };
        (Missile::new(), body)
    }

    pub fn update(&self, body: &mut Body) {
        let rotation = self.rotation as i32 as f32;
        body.rot = Rad::full_turn() * 0.4 * rotation;
    }
}
