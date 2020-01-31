use crate::{
    cgs::Store,
    game::{
        crafts::Craft,
        physics::Body,
    }
};

use super::{
    Ship,
    update_ships,
};


pub struct Feature {
    pub ships: Store<Ship>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            ships: Store::new(),
        }
    }

    pub fn on_update(&mut self,
        bodies: &mut Store<Body>,
        crafts: &Store<Craft>,
    ) {
        update_ships(
            bodies,
            crafts,
            &mut self.ships,
        );
    }
}
