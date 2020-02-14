use toadster::store;

use crate::game::{
    base::Update,
    physics::{
        Body,
        Direction,
    },
};

use super::{
    Craft,
    Fuel,
    update_crafts,
};


pub struct Feature {
    pub crafts: store::Strong<Craft>,
    pub fuels:  store::Strong<Fuel>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            crafts: store::Strong::new(),
            fuels:  store::Strong::new(),
        }
    }

    pub fn on_update(&mut self,
        event:      &Update,
        bodies:     &mut store::Strong<Body>,
        directions: &store::Strong<Direction>,
    ) {
        update_crafts(
            bodies,
            &mut self.crafts,
            directions,
            &mut self.fuels,
            event.dt,
        );
    }
}
