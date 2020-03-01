use toadster::store;

use crate::game::base::Update;

use super::{
    Body,
    Direction,
    Position,
    Velocity,
    update_bodies,
};


pub struct Feature {
    pub velocities: store::Strong<Velocity>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            velocities: store::Strong::new(),
        }
    }

    pub fn on_update(&mut self,
        event:      &Update,
        world_size: f32,
        bodies:     &mut store::Strong<Body>,
        directions: &mut store::Strong<Direction>,
        positions:  &mut store::Strong<Position>,
    ) {
        update_bodies(
            bodies,
            directions,
            positions,
            &mut self.velocities,
            world_size,
            event.dt,
        );
    }
}
