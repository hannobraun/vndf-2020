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
    pub fuels: store::Strong<Fuel>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            fuels: store::Strong::new(),
        }
    }

    pub fn on_update(&mut self,
        event:      &Update,
        bodies:     &mut store::Strong<Body>,
        crafts:     &mut store::Strong<Craft>,
        directions: &store::Strong<Direction>,
    ) {
        update_crafts(
            bodies,
            crafts,
            directions,
            &mut self.fuels,
            event.dt,
        );
    }
}
