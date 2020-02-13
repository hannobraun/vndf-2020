use crate::{
    cgs::{
        Handle,
        Store,
    },
    game::{
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
    },
    events,
};

use super::{
    components::Health,
    events::Death,
};


pub fn check_health(
    healths: &Store<Health>,
    death:   &mut events::Sink<Death>,
) {
    for (handle, health) in healths {
        if health.is_dead() {
            death.push(Death { handle });
        }
    }
}

pub fn remove_entity(
    handle:     &Handle<Health>,
    bodies:     &mut Store<Body>,
    crafts:     &mut Store<Craft>,
    directions: &mut Store<Direction>,
    fuels:      &mut Store<Fuel>,
    guidances:  &mut Store<Guidance>,
    healths:    &mut Store<Health>,
    missiles:   &mut Store<Missile>,
    positions:  &mut Store<Position>,
    ships:      &mut Store<Ship>,
    targets:    &mut Store<Target>,
    velocities: &mut Store<Velocity>,
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
