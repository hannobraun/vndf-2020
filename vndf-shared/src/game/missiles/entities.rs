use crate::{
    cgs::Store,
    game::{
        base::ComponentHandle,
        crafts::{
            Craft,
            Fuel,
        },
        health::Health,
        physics::{
            Body,
            Direction,
            Position,
            Velocity,
        },
        players::PlayerId,
    },
    math::{
        prelude::*,
        Pnt2,
        Rad,
    },
};

use super::{
    Guidance,
    Missile,
    Target,
};


pub struct MissileEntity {
    pub owner:  PlayerId,
    pub origin: Body,
    pub target: Pnt2,
}

impl MissileEntity {
    pub fn create(&self,
        bodies:     &mut Store<Body>,
        crafts:     &mut Store<Craft>,
        directions: &mut Store<Direction>,
        fuels:      &mut Store<Fuel>,
        guidances:  &mut Store<Guidance>,
        healths:    &mut Store<Health>,
        missiles:   &mut Store<Missile>,
        positions:  &mut Store<Position>,
        targets:    &mut Store<Target>,
        velocities: &mut Store<Velocity>,
    )
        -> Option<()>
    {
        const FUEL:   f32 = 1600.0;
        const HEALTH: f32 =    2.0;
        const THRUST: f32 =  200.0;

        let pos       = *positions.get(self.origin.pos)?;
        let to_target = self.target - pos.0;
        let pos       = positions.insert(pos);

        let vel = *velocities.get(self.origin.vel)?;
        let vel = velocities.insert(vel);

        let dir = directions.insert(Direction(to_target));

        let body = Body {
            pos,
            vel,
            dir,
            rot: Rad::zero(),
            .. self.origin
        };
        let body = bodies.insert(body);

        let fuel   = fuels.insert(Fuel(FUEL));
        let health = healths.insert(Health::new(body, HEALTH));

        let craft = Craft {
            body,
            fuel,
            health,

            engine_on: true,
            thrust:    THRUST,
            owner:     self.owner,
        };
        let craft = crafts.insert(craft);

        let target = Target { craft, value: self.target };
        let target = targets.insert(target);

        let guidance = Guidance::new(craft, target);
        let guidance = guidances.insert(guidance);

        let missile = missiles.insert(Missile::new(craft, guidance, target));
        healths.get_mut(health).unwrap().parent =
            Some(ComponentHandle::Missile(missile));

        Some(())
    }
}
