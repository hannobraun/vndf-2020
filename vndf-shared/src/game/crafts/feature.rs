use toadster::StrongStore;

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
    pub crafts: StrongStore<Craft>,
    pub fuels:  StrongStore<Fuel>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            crafts: StrongStore::new(),
            fuels:  StrongStore::new(),
        }
    }

    pub fn on_update(&mut self,
        event:      &Update,
        bodies:     &mut StrongStore<Body>,
        directions: &StrongStore<Direction>,
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
