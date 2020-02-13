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
    handle:     &StrongHandle<Health>,
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
    let health = healths.get(handle)?;
    let parent = health.parent?;

    if let ComponentHandle::Missile(handle) = parent {
        Missile::remove(
            handle,
            bodies,
            crafts,
            directions,
            fuels,
            guidances,
            healths,
            missiles,
            positions,
            targets,
            velocities,
        );
    }
    if let ComponentHandle::Ship(handle) = parent {
        Ship::remove(
            handle,
            bodies,
            crafts,
            directions,
            fuels,
            healths,
            positions,
            ships,
            velocities,
        );
    }

    Some(())
}
