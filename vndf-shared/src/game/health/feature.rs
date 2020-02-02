use crate::{
    cgs::Store,
    events,
    game::{
        crafts::Craft,
        missiles::Missile,
        physics::Body,
        ships::Ship,
    }
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
        event:    &Death,
        bodies:   &mut Store<Body>,
        crafts:   &mut Store<Craft>,
        missiles: &mut Store<Missile>,
        ships:    &mut Store<Ship>,
    ) {
        remove_entity(
            event.handle,
            bodies,
            crafts,
            &mut self.healths,
            missiles,
            ships,
        );
    }
}
