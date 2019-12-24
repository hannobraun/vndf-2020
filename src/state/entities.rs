use cgmath::prelude::*;

use crate::{
    math::Rad,
    state::components::{
        Body,
        Engine,
        Explosion,
        Missile,
        Ship,
    },
};


pub fn explosion(exploding: &Body) -> (Explosion, Body) {
    let body = Body {
        pos: exploding.pos,
        vel: exploding.vel * 0.1,
        .. Body::new()
    };

    (Explosion::new(), body)
}

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
