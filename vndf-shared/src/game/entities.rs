use std::net::SocketAddr;

use crate::{
    game::{
        PlayerId,
        components as c,
        features::health::components::Health,
    },
    math::{
        prelude::*,
        Pnt2,
        Rad,
    },
};


pub type Explosion = (c::Explosion, c::Body);
pub type Missile   = (c::Missile, c::Body, c::Craft, Health);
pub type Player    = (c::Player,);
pub type Ship      = (c::Ship, c::Body, c::Craft, Health);


pub fn explosion(exploding: &c::Body, strength: f32) -> Explosion {
    let body = c::Body {
        pos: exploding.pos,
        vel: exploding.vel * 0.05,
        .. c::Body::new()
    };

    (c::Explosion::new(strength), body)
}

pub fn missile(owner: PlayerId, from_body: &c::Body, target: Pnt2) -> Missile {
    let to_target = target - from_body.pos;

    let body = c::Body {
        dir: to_target,
        rot: Rad::zero(),
        .. *from_body
    };
    let craft = c::Craft {
        engine_on: true,
        thrust:    200.0,
        fuel:      400.0,
        owner,
    };

    (c::Missile::new(target), body, craft, Health::new(2.0))
}

pub fn player(id: PlayerId, addr: SocketAddr) -> Player {
    (c::Player::new(id, addr),)
}

pub fn ship(owner: PlayerId, color: [f32; 3]) -> Ship {
    let craft = c::Craft {
        engine_on: false,
        thrust:    100.0,
        fuel:      1200.0,
        owner,
    };

    (c::Ship::new(color), c::Body::new(), craft, Health::new(10.0))
}
