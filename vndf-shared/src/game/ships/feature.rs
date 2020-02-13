use toadster::StrongStore;

use crate::game::{
    crafts::Craft,
    physics::Body,
};

use super::{
    Ship,
    update_ships,
};


pub struct Feature {
    pub ships: StrongStore<Ship>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            ships: StrongStore::new(),
        }
    }

    pub fn on_update(&mut self,
        bodies: &mut StrongStore<Body>,
        crafts: &StrongStore<Craft>,
    ) {
        update_ships(
            bodies,
            crafts,
            &mut self.ships,
        );
    }
}
