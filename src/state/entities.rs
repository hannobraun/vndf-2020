use cgmath::prelude::*;

use crate::{
    math::Rad,
    state::components::{
        Body,
        Engine,
        Missile,
        Ship,
    },
};


pub fn missile(launcher: &Body) -> (Missile, Body, Engine) {
    let body = Body {
        rot: Rad::zero(),
        .. *launcher
    };
    let engine = Engine {
        enabled: true,
    };

    (Missile::new(), body, engine)
}

pub fn ship() -> (Ship, Body, Engine) {
    (Ship::new(), Body::new(), Engine::new())
}
