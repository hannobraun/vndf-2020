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
    pub directions: store::Strong<Direction>,
    pub positions:  store::Strong<Position>,
    pub velocities: store::Strong<Velocity>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            directions: store::Strong::new(),
            positions:  store::Strong::new(),
            velocities: store::Strong::new(),
        }
    }

    pub fn on_update(&mut self,
        event:      &Update,
        world_size: f32,
        bodies:     &mut store::Strong<Body>,
    ) {
        update_bodies(
            bodies,
            &mut self.directions,
            &mut self.positions,
            &mut self.velocities,
            world_size,
            event.dt,
        );
    }
}
