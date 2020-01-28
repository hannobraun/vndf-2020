use crate::{
    cgs::{
        Handle,
        Store,
    },
    game::{
        base::ComponentHandle,
        crafts::Craft,
        missiles::Missile,
        physics::Body,
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
    handle:   Handle,
    bodies:   &mut Store<Body>,
    crafts:   &mut Store<Craft>,
    healths:  &mut Store<Health>,
    missiles: &mut Store<Missile>,
    ships:    &mut Store<Ship>,
)
    -> Option<()>
{
    let health = healths.get(handle)?;
    let parent = health.parent?;

    if let ComponentHandle::Missile(handle) = parent {
        Missile::remove(handle, bodies, crafts, healths, missiles);
    }
    if let ComponentHandle::Ship(handle) = parent {
        Ship::remove(handle, bodies, crafts, healths, ships);
    }

    Some(())
}
