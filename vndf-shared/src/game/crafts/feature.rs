use crate::{
    cgs::Store,
    game::{
        base::Update,
        physics::Body,
    },
};

use super::{
    Craft,
    update_crafts,
};


pub struct Feature {
    pub crafts: Store<Craft>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            crafts: Store::new(),
        }
    }

    pub fn on_update(&mut self, event: &Update, bodies: &mut Store<Body>) {
        update_crafts(
            bodies,
            &mut self.crafts,
            event.dt,
        );
    }
}
