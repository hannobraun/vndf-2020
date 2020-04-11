use toadster::store;

use crate::world::{
    base::Update,
    planets::Planet,
};

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
        bodies:     &mut store::Strong<Body>,
        planets:    &store::Strong<Planet>,
        positions:  &mut store::Strong<Position>,
        velocities: &mut store::Strong<Velocity>,
    ) {
        update_bodies(
            bodies,
            planets,
            positions,
            velocities,
            event.dt,
        );
    }
}
