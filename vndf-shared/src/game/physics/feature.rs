use crate::{
    cgs::Store,
    game::base::Update,
};

use super::{
    Body,
    Direction,
    Position,
    Velocity,
    update_bodies,
};


pub struct Feature {
    pub bodies:     Store<Body>,
    pub directions: Store<Direction>,
    pub positions:  Store<Position>,
    pub velocities: Store<Velocity>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            bodies:     Store::new(),
            directions: Store::new(),
            positions:  Store::new(),
            velocities: Store::new(),
        }
    }

    pub fn on_update(&mut self, event: &Update, world_size: f32) {
        update_bodies(
            &mut self.bodies,
            &mut self.directions,
            &mut self.positions,
            &mut self.velocities,
            world_size,
            event.dt,
        );
    }
}
