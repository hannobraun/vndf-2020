use toadster::store;

use crate::game::{
    crafts::Craft,
    physics::Body,
};

use super::{
    Ship,
    update_ships,
};


pub struct Feature;

impl Feature {
    pub fn new() -> Self {
        Self
    }

    pub fn on_update(&mut self,
        bodies: &mut store::Strong<Body>,
        crafts: &store::Strong<Craft>,
        ships:  &mut store::Strong<Ship>,
    ) {
        update_ships(
            bodies,
            crafts,
            ships,
        );
    }
}
