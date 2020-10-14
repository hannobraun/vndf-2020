use toadster::store;

use crate::world::{base::Update, physics::Body};

use super::{update_crafts, Craft, Fuel};

pub struct Feature;

impl Feature {
    pub fn new() -> Self {
        Self
    }

    pub fn on_update(
        &mut self,
        event: &Update,
        bodies: &mut store::Strong<Body>,
        crafts: &mut store::Strong<Craft>,
        fuels: &mut store::Strong<Fuel>,
    ) {
        update_crafts(bodies, crafts, fuels, event.dt);
    }
}
