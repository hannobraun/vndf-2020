use cgmath::prelude::*;

use crate::{
    math::Rad,
    state::components as c,
};


pub type Explosion = (c::Explosion, c::Body);
pub type Missile   = (c::Missile, c::Body, c::Engine);
pub type Ship      = (c::Ship, c::Body, c::Engine);


pub fn explosion(exploding: &c::Body) -> Explosion {
    let body = c::Body {
        pos: exploding.pos,
        vel: exploding.vel * 0.1,
        .. c::Body::new()
    };

    (c::Explosion::new(), body)
}

pub fn missile(launcher: &c::Body) -> Missile {
    let body = c::Body {
        rot: Rad::zero(),
        .. *launcher
    };
    let engine = c::Engine {
        enabled: true,
        thrust:  10.0,
        fuel:    80.0
    };

    (c::Missile::new(), body, engine)
}

pub fn ship() -> Ship {
    let engine = c::Engine {
        enabled: false,
        thrust:  5.0,
        fuel:    300.0,
    };

    (c::Ship::new(), c::Body::new(), engine)
}
