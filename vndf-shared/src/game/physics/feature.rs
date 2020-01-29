use crate::cgs::Store;

use super::{
    Body,
    update_bodies,
};


pub struct Feature {
    pub bodies: Store<Body>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            bodies: Store::new(),
        }
    }

    pub fn on_update(&mut self, world_size: f32, dt: f32) {
        update_bodies(
            &mut self.bodies,
            world_size,
            dt,
        );
    }
}
