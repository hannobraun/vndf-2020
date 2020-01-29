use crate::{
    cgs::Store,
    game::{
        base::ComponentHandle,
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
};

use super::Missile;


pub struct MissileEntity {
    pub owner:  PlayerId,
    pub origin: Body,
    pub target: Pnt2,
}

impl MissileEntity {
    pub fn create(&self,
        bodies:   &mut Store<Body>,
        crafts:   &mut Store<Craft>,
        healths:  &mut Store<Health>,
        missiles: &mut Store<Missile>,
    ) {
        let to_target = self.target - self.origin.pos;
        let body = Body {
            dir: to_target,
            rot: Rad::zero(),
            .. self.origin
        };
        let body = bodies.insert(body);

        let health = healths.insert(Health::new(body, 2.0));

        let craft = Craft {
            body,
            health,

            engine_on: true,
            thrust:    200.0,
            fuel:      1600.0,
            owner:     self.owner,
        };
        let craft = crafts.insert(craft);

        let missile = missiles.insert(Missile::new(craft, self.target));
        healths.get_mut(health).unwrap().parent =
            Some(ComponentHandle::Missile(missile));
    }
}
