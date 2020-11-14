use toadster::store;

use crate::world::{crafts::Craft, features::base::Update, physics::Body};

use super::{update_ships, Ship};

pub struct Feature;

impl Feature {
    pub fn new() -> Self {
        Self
    }

    pub fn on_update(
        &mut self,
        event: &Update,
        bodies: &mut store::Strong<Body>,
        crafts: &store::Strong<Craft>,
        ships: &mut store::Strong<Ship>,
    ) {
        update_ships(event.dt, bodies, crafts, ships);
    }
}
