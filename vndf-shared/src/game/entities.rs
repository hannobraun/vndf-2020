use std::net::SocketAddr;

use crate::{
    game::{
        PlayerId,
        components as c,
        features::{
            crafts::components::Craft,
            explosions::components::Explosion,
            health::components::Health,
            missiles::components::Missile,
            physics::components::Body,
        },
    },
    math::{
        prelude::*,
        Pnt2,
        Rad,
    },
};


pub type ExplosionE = (Explosion, Body);
pub type MissileE   = (Missile, Body, Craft, Health);
pub type PlayerE    = (c::Player,);
pub type ShipE      = (c::Ship, Body, Craft, Health);


pub fn explosion(exploding: &Body, strength: f32) -> ExplosionE {
    let body = Body {
        pos: exploding.pos,
        vel: exploding.vel * 0.05,
        .. Body::new()
    };

    (Explosion::new(strength), body)
}

pub fn missile(owner: PlayerId, from_body: &Body, target: Pnt2) -> MissileE {
    let to_target = target - from_body.pos;

    let body = Body {
        dir: to_target,
        rot: Rad::zero(),
        .. *from_body
    };
    let craft = Craft {
        engine_on: true,
        thrust:    200.0,
        fuel:      400.0,
        owner,
    };

    (Missile::new(target), body, craft, Health::new(2.0))
}

pub fn player(id: PlayerId, addr: SocketAddr) -> PlayerE {
    (c::Player::new(id, addr),)
}

pub fn ship(owner: PlayerId, color: [f32; 3]) -> ShipE {
    let craft = Craft {
        engine_on: false,
        thrust:    100.0,
        fuel:      1200.0,
        owner,
    };

    (c::Ship::new(color), Body::new(), craft, Health::new(10.0))
}
