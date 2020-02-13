use log::warn;
use toadster::{
    StrongHandle,
    StrongStore,
};
use vndf_events as events;

use crate::game::{
    base::ComponentHandle,
    crafts::{
        Craft,
        Fuel,
    },
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
    healths: &StrongStore<Health>,
    death:   &mut events::Sink<Death>,
) {
    for (handle, health) in healths {
        if health.is_dead() {
            death.push(Death { handle });
        }
    }
}

pub fn remove_entity(
    handle:     StrongHandle<Health>,
    bodies:     &mut StrongStore<Body>,
    crafts:     &mut StrongStore<Craft>,
    directions: &mut StrongStore<Direction>,
    fuels:      &mut StrongStore<Fuel>,
    guidances:  &mut StrongStore<Guidance>,
    healths:    &mut StrongStore<Health>,
    missiles:   &mut StrongStore<Missile>,
    positions:  &mut StrongStore<Position>,
    ships:      &mut StrongStore<Ship>,
    targets:    &mut StrongStore<Target>,
    velocities: &mut StrongStore<Velocity>,
)
    -> Option<()>
{
    let health = healths.remove(handle)?;

    match health.parent? {
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
