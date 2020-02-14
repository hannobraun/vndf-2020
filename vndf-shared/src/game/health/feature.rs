use bach::EventBuf;
use toadster::StrongStore;

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
    pub healths: StrongStore<Health>,
    pub death:   EventBuf<Death>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            healths: StrongStore::new(),
            death:   EventBuf::new(),
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
        bodies:     &mut StrongStore<Body>,
        crafts:     &mut StrongStore<Craft>,
        directions: &mut StrongStore<Direction>,
        fuels:      &mut StrongStore<Fuel>,
        guidances:  &mut StrongStore<Guidance>,
        missiles:   &mut StrongStore<Missile>,
        positions:  &mut StrongStore<Position>,
        ships:      &mut StrongStore<Ship>,
        targets:    &mut StrongStore<Target>,
        velocities: &mut StrongStore<Velocity>,
    ) {
        remove_entity(
            event.handle,
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
