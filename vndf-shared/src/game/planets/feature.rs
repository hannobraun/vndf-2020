use toadster::store;

use crate::{
    game::{
        health::Health,
        physics::components::{
            Body,
            Position,
        },
    },
};

use super::{
    Planet,
    apply_gravitation,
    check_collision,
};


pub struct Feature;

impl Feature {
    pub fn new() -> Self {
        Self
    }

    pub fn on_update(&self,
        bodies:    &mut store::Strong<Body>,
        healths:   &mut store::Strong<Health>,
        planets:   &store::Strong<Planet>,
        positions: &store::Strong<Position>,
    ) {
        apply_gravitation(
            bodies,
            planets,
            positions,
        );
        check_collision(
            bodies,
            healths,
            planets,
            positions,
        );
    }
}
