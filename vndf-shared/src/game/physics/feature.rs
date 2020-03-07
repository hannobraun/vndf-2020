use toadster::store;

use crate::game::base::Update;

use super::{
    Body,
    Position,
    Velocity,
    update_bodies,
};


pub struct Feature;

impl Feature {
    pub fn new() -> Self {
        Self
    }

    pub fn on_update(&mut self,
        event:      &Update,
        world_size: f32,
        bodies:     &mut store::Strong<Body>,
        positions:  &mut store::Strong<Position>,
        velocities: &mut store::Strong<Velocity>,
    ) {
        update_bodies(
            bodies,
            positions,
            velocities,
            world_size,
            event.dt,
        );
    }
}
