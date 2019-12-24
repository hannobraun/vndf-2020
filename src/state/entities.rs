use cgmath::prelude::*;

use crate::{
    math::Rad,
    state::components as c,
};


pub fn explosion(exploding: &c::Body) -> (c::Explosion, c::Body) {
    let body = c::Body {
        pos: exploding.pos,
        vel: exploding.vel * 0.1,
        .. c::Body::new()
    };

    (c::Explosion::new(), body)
}

pub fn missile(launcher: &c::Body) -> (c::Missile, c::Body, c::Engine) {
    let body = c::Body {
        rot: Rad::zero(),
        .. *launcher
    };
    let engine = c::Engine {
        enabled: true,
        thrust:  50.0,
        fuel:    500.0
    };

    (c::Missile::new(), body, engine)
}

pub fn ship() -> (c::Ship, c::Body, c::Engine) {
    let engine = c::Engine {
        enabled: false,
        thrust:  20.0,
        fuel:    7200.0,
    };

    (c::Ship::new(), c::Body::new(), engine)
}
