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


pub struct Feature<'r> {
    pub bodies:    &'r mut store::Strong<Body>,
    pub healths:   &'r mut store::Strong<Health>,
    pub planets:   &'r store::Strong<Planet>,
    pub positions: &'r store::Strong<Position>,
}

impl Feature<'_> {
    pub fn on_update(&mut self) {
        apply_gravitation(
            self.bodies,
            self.planets,
            self.positions,
        );
        check_collision(
            self.bodies,
            self.healths,
            self.planets,
            self.positions,
        );
    }
}
