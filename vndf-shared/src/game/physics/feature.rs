use crate::{
    cgs::Store,
    game::base::Update,
};

use super::{
    Body,
    Position,
    update_bodies,
};


pub struct Feature {
    pub bodies:    Store<Body>,
    pub positions: Store<Position>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            bodies:    Store::new(),
            positions: Store::new(),
        }
    }

    pub fn on_update(&mut self, event: &Update, world_size: f32) {
        update_bodies(
            &mut self.bodies,
            world_size,
            event.dt,
        );
    }
}
