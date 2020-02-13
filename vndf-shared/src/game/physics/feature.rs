use toadster::StrongStore;

use crate::game::base::Update;

use super::{
    Body,
    Direction,
    Position,
    Velocity,
    update_bodies,
};


pub struct Feature {
    pub bodies:     StrongStore<Body>,
    pub directions: StrongStore<Direction>,
    pub positions:  StrongStore<Position>,
    pub velocities: StrongStore<Velocity>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            bodies:     StrongStore::new(),
            directions: StrongStore::new(),
            positions:  StrongStore::new(),
            velocities: StrongStore::new(),
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
