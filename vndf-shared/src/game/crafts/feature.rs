use crate::{
    cgs::Store,
    game::{
        base::Update,
        physics::{
            Body,
            Direction,
        },
    },
};

use super::{
    Craft,
    Fuel,
    update_crafts,
};


pub struct Feature {
    pub crafts: Store<Craft>,
    pub fuels:  Store<Fuel>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            crafts: Store::new(),
            fuels:  Store::new(),
        }
    }

    pub fn on_update(&mut self,
        event:      &Update,
        bodies:     &mut Store<Body>,
        directions: &Store<Direction>,
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
