use crate::{
    game::{
        crafts::Craft,
        health::Health,
        physics::Body,
        players::PlayerId,
        missiles::components::Missile,
    },
    math::{
        prelude::*,
        Pnt2,
        Rad,
    },
    world,
};


pub struct MissileEntity {
    pub owner:  PlayerId,
    pub origin: Body,
    pub target: Pnt2,
}

impl MissileEntity {
    pub fn create(&self, world: &mut world::Spawn) {
        let to_target = self.target - self.origin.pos;

        let body = Body {
            dir: to_target,
            rot: Rad::zero(),
            .. self.origin
        };
        let craft = Craft {
            engine_on: true,
            thrust:    200.0,
            fuel:      400.0,
            owner:     self.owner,
        };

        world.spawn((Missile::new(self.target), body, craft, Health::new(2.0)));
    }
}
