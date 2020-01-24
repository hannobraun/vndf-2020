use crate::{
    game::{
        PlayerId,
        features::{
            crafts::components::Craft,
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


pub type MissileE   = (Missile, Body, Craft, Health);


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
