use std::collections::HashSet;

use log::warn;
use rinnsal::EventSink;
use toadster::{
    handle::{
        self,
        Untyped,
    },
    store,
};

use crate::game::{
    base::ComponentHandle,
    crafts::{
        Craft,
        Fuel,
    },
    loot::Loot,
    missiles::{
        Guidance,
        Missile,
        Target,
    },
    physics::{
        Body,
        Direction,
        Position,
        Velocity,
    },
    ships::Ship,
};

use super::{
    components::Health,
    events::Death,
};


pub fn check_health(
    healths: &store::Strong<Health>,
    death:   &mut EventSink<Death>,
    index:   &mut HashSet<handle::Strong<Untyped>>,
) {
    for (handle, health) in healths {
        if health.is_dead() {
            let parent = health
                .parent_ref()
                .unwrap()
                .clone()
                .into_weak_untyped();
            index.remove(&parent);

            death.push(Death { handle });
        }
    }
}

pub fn remove_entity(
    handle:     handle::Strong<Health>,
    bodies:     &mut store::Strong<Body>,
    crafts:     &mut store::Strong<Craft>,
    directions: &mut store::Strong<Direction>,
    fuels:      &mut store::Strong<Fuel>,
    guidances:  &mut store::Strong<Guidance>,
    healths:    &mut store::Strong<Health>,
    loots:      &mut store::Strong<Loot>,
    missiles:   &mut store::Strong<Missile>,
    positions:  &mut store::Strong<Position>,
    ships:      &mut store::Strong<Ship>,
    targets:    &mut store::Strong<Target>,
    velocities: &mut store::Strong<Velocity>,
)
    -> Option<()>
{
    let health = healths.remove(handle)?;

    match health.parent()? {
        ComponentHandle::Loot(handle) => {
            loots.remove(handle);
        }
        ComponentHandle::Missile(handle) => {
            Missile::remove(
                handle,
                bodies,
                crafts,
                directions,
                fuels,
                guidances,
                missiles,
                positions,
                targets,
                velocities,
            );
        }
        ComponentHandle::Ship(handle) => {
            Ship::remove(
                handle,
                bodies,
                crafts,
                directions,
                fuels,
                positions,
                ships,
                velocities,
            );
        }
        _ => {
            warn!("Dead entity is neither ship nor missile");
        }
    }

    Some(())
}
