use std::net::SocketAddr;

use crate::{
    game::{
        PlayerId,
        components as c,
    },
    math::{
        prelude::*,
        Pnt2,
        Rad,
    },
};


pub type Explosion = (c::Explosion, c::Body);
pub type Missile   = (c::Missile, c::Body, c::Craft);
pub type Player    = (c::Player,);
pub type Ship      = (c::Ship, c::Body, c::Craft);


pub fn explosion(exploding: &c::Body) -> Explosion {
    let body = c::Body {
        pos: exploding.pos,
        vel: exploding.vel * 0.1,
        .. c::Body::new()
    };

    (c::Explosion::new(), body)
}

pub fn missile(owner: PlayerId, launcher: &c::Body, target: Pnt2) -> Missile {
    let to_target = target - launcher.pos;

    let body = c::Body {
        dir: to_target,
        rot: Rad::zero(),
        .. *launcher
    };
    let craft = c::Craft {
        engine_on: true,
        thrust:    200.0,
        fuel:      400.0,
        owner,
    };

    (c::Missile::new(target), body, craft)
}

pub fn player(id: PlayerId) -> Player {
    (c::Player::new(id),)
}

pub fn ship(owner: PlayerId, player: SocketAddr) -> Ship {
    let craft = c::Craft {
        engine_on: false,
        thrust:    100.0,
        fuel:      1200.0,
        owner,
    };

    (c::Ship::new(player), c::Body::new(), craft)
}
