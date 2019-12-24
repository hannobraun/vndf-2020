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
        thrust:  50.0,
        fuel:    500.0
    };

    (Missile::new(), body, engine)
}

pub fn ship() -> (Ship, Body, Engine) {
    let engine = Engine {
        enabled: false,
        thrust:  20.0,
        fuel:    7200.0,
    };

    (Ship::new(), Body::new(), engine)
}
