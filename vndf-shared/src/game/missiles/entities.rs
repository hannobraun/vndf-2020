use toadster::store;

use crate::{
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
        bodies:     &mut store::Strong<Body>,
        crafts:     &mut store::Strong<Craft>,
        directions: &mut store::Strong<Direction>,
        fuels:      &mut store::Strong<Fuel>,
        guidances:  &mut store::Strong<Guidance>,
        healths:    &mut store::Strong<Health>,
        missiles:   &mut store::Strong<Missile>,
        positions:  &mut store::Strong<Position>,
        targets:    &mut store::Strong<Target>,
        velocities: &mut store::Strong<Velocity>,
    )
        -> Option<()>
    {
        const THRUST: f32 =   10.0;
        const FUEL:   f32 = 1600.0;
        const HEALTH: f32 =    2.0;

        let pos       = *positions.get(&self.origin.pos)?;
        let to_target = self.target - pos.0;
        let pos       = positions.insert(pos);

        let vel = *velocities.get(&self.origin.vel)?;
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
        let health = healths.insert(Health::new(body.clone(), HEALTH));

        let craft = Craft {
            body,
            fuel,
            health: health.clone(),

            engine_on: true,
            thrust:    THRUST,
            owner:     self.owner,
        };
        let craft = crafts.insert(craft);

        let target = Target { craft: craft.clone(), value: self.target };
        let target = targets.insert(target);

        let guidance = Guidance::new(craft.clone(), target.clone());
        let guidance = guidances.insert(guidance);

        let missile = missiles.insert(Missile::new(craft, guidance, target));
        healths.get_mut(&health).unwrap().parent =
            Some(ComponentHandle::Missile(missile.into()));

        Some(())
    }
}
