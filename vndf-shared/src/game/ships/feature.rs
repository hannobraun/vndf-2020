use toadster::store;

use crate::game::{
    crafts::Craft,
    physics::Body,
};

use super::{
    Ship,
    update_ships,
};


pub struct Feature {
    pub ships: store::Strong<Ship>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            ships: store::Strong::new(),
        }
    }

    pub fn on_update(&mut self,
        bodies: &mut store::Strong<Body>,
        crafts: &store::Strong<Craft>,
    ) {
        update_ships(
            bodies,
            crafts,
            &mut self.ships,
        );
    }
}
