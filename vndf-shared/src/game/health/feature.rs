use toadster::Store;
use vndf_events as events;

use crate::game::{
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
    Death,
    Health,
    check_health,
    remove_entity,
};


pub struct Feature {
    pub healths: Store<Health>,
    pub death:   events::Buf<Death>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            healths: Store::new(),
            death:   events::Buf::new(),
        }
    }

    pub fn on_update(&mut self) {
        check_health(
            &self.healths,
            &mut self.death.sink(),
        );
    }

    pub fn on_death(&mut self,
        event:      &Death,
        bodies:     &mut Store<Body>,
        crafts:     &mut Store<Craft>,
        directions: &mut Store<Direction>,
        fuels:      &mut Store<Fuel>,
        guidances:  &mut Store<Guidance>,
        missiles:   &mut Store<Missile>,
        positions:  &mut Store<Position>,
        ships:      &mut Store<Ship>,
        targets:    &mut Store<Target>,
        velocities: &mut Store<Velocity>,
    ) {
        remove_entity(
            &event.handle,
            bodies,
            crafts,
            directions,
            fuels,
            guidances,
            &mut self.healths,
            missiles,
            positions,
            ships,
            targets,
            velocities,
        );
    }
}
