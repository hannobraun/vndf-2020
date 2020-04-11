use std::collections::HashSet;

use toadster::{
    handle::{
        self,
        Untyped,
    },
    store,
};

use crate::{
    data,
    math::{
        Angle,
        Pnt2,
    },
    world::{
        crafts::{
            Craft,
            Fuel,
        },
        health::Health,
        physics::{
            Body,
            Position,
            Velocity,
        },
        players::PlayerId,
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
        fuels:      &mut store::Strong<Fuel>,
        guidances:  &mut store::Strong<Guidance>,
        healths:    &mut store::Strong<Health>,
        missiles:   &mut store::Strong<Missile>,
        positions:  &mut store::Strong<Position>,
        targets:    &mut store::Strong<Target>,
        velocities: &mut store::Strong<Velocity>,
        entities:   &mut HashSet<handle::Strong<Untyped>>,
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

        let body = Body {
            pos: pos.into(),
            vel: vel.into(),
            dir: to_target,
            rot: Angle::zero(),
            .. self.origin
        };
        let body = bodies.insert(body);

        let fuel   = fuels.insert(Fuel(FUEL));
        let health = healths.insert(Health::new(body.clone(), HEALTH));

        let craft = Craft {
            body:   body.into(),
            fuel:   fuel.into(),
            health: health.clone().into(),

            engine_on: true,
            thrust:    THRUST,
            owner:     self.owner,
        };
        let craft = crafts.insert(craft);

        let target = Target { craft: craft.clone().into(), value: self.target };
        let target = targets.insert(target);

        let guidance = Guidance::new(craft.clone(), target.clone());
        let guidance = guidances.insert(guidance);

        let missile = missiles.insert(Missile::new(craft, guidance, target));
        healths.get_mut(&health).unwrap().finalize(
            data::client::Handle::Missile(missile.into()),
            entities,
        );

        Some(())
    }
}
