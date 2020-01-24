use crate::{
    cgs::Store,
    game::{
        crafts::Craft,
        health::Health,
        physics::Body,
        players::PlayerId,
    },
    math::{
        prelude::*,
        Pnt2,
        Rad,
    },
    world,
};

use super::Missile;


pub struct MissileEntity {
    pub owner:  PlayerId,
    pub origin: Body,
    pub target: Pnt2,
}

impl MissileEntity {
    pub fn create(&self,
        world:    &mut world::Spawn,
        missiles: &mut Store<Missile>,
    ) {
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

        let entity = world.spawn((body, craft, Health::new(2.0)));
        missiles.insert(Missile::new(entity, self.target));
    }
}
