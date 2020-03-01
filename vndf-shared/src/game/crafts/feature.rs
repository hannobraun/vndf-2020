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


pub struct Feature;

impl Feature {
    pub fn new() -> Self {
        Self
    }

    pub fn on_update(&mut self,
        event:      &Update,
        bodies:     &mut store::Strong<Body>,
        crafts:     &mut store::Strong<Craft>,
        directions: &store::Strong<Direction>,
        fuels:      &mut store::Strong<Fuel>,
    ) {
        update_crafts(
            bodies,
            crafts,
            directions,
            fuels,
            event.dt,
        );
    }
}
