use std::net::SocketAddr;

use crate::{
    game::components as c,
    math::{
        prelude::*,
        Pnt2,
        Rad,
    },
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

pub fn missile(launcher: &c::Body, target: Pnt2) -> Missile {
    let to_target = target - launcher.pos;

    let body = c::Body {
        dir: to_target,
        rot: Rad::zero(),
        .. *launcher
    };
    let engine = c::Engine {
        enabled: true,
        thrust:  200.0,
        fuel:    400.0,
    };

    (c::Missile::new(target), body, engine)
}

pub fn ship(player: SocketAddr) -> Ship {
    let engine = c::Engine {
        enabled: false,
        thrust:  100.0,
        fuel:    1200.0,
    };

    (c::Ship::new(player), c::Body::new(), engine)
}
