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
    game::{
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
        ships::Ship,
    },
};

use super::Loot;


pub fn spawn_death_loot(
    handle:     &handle::Strong<Health>,
    bodies:     &mut store::Strong<Body>,
    crafts:     &store::Strong<Craft>,
    fuels:      &store::Strong<Fuel>,
    healths:    &mut store::Strong<Health>,
    loots:      &mut store::Strong<Loot>,
    positions:  &mut store::Strong<Position>,
    ships:      &store::Strong<Ship>,
    velocities: &mut store::Strong<Velocity>,
    index:      &mut HashSet<handle::Strong<Untyped>>,
)
    -> Option<()>
{
    let health = healths.get(handle)?;

    if let data::client::Handle::Ship(handle) = health.parent_ref()? {
        let ship  = ships.get(handle)?;
        let craft = crafts.get(&ship.craft)?;
        let fuel  = fuels.get(&craft.fuel)?;
        let body  = bodies.get(&craft.body)?;
        let pos   = positions.get(&body.pos)?;
        let vel   = velocities.get(&body.vel)?;

        let pos = *pos;
        let pos = positions.insert(pos);

        let vel = *vel;
        let vel = velocities.insert(vel);

        let body = bodies.insert(Body::new(pos, vel));

        let health = healths.insert(Health::new(body.clone(), 1.0));

        let loot = Loot {
            body:     body.into(),
            health:   health.clone().into(),
            fuel:     fuel.0 / 10.0,
            missiles: ship.missiles / 10,
        };

        let loot = loots.insert(loot);
        healths.get_mut(health).unwrap().finalize(
            data::client::Handle::Loot(loot.into()),
            index,
        );
    }

    Some(())
}

pub fn collect_loot(
    bodies:    &store::Strong<Body>,
    crafts:    &store::Strong<Craft>,
    fuels:     &mut store::Strong<Fuel>,
    healths:   &mut store::Strong<Health>,
    loots:     &store::Strong<Loot>,
    positions: &store::Strong<Position>,
    ships:     &mut store::Strong<Ship>,
) {
    for (handle, loot) in loots {
        loot.collect(
            handle,
            bodies,
            crafts,
            fuels,
            healths,
            loots,
            positions,
            ships,
        );
    }
}
