use toadster::store;

use crate::{
    game::{
        health::Health,
        physics::components::{
            Body,
            Position,
        },
    },
    math::Pnt2,
};

use super::{
    Planet,
    apply_gravitation,
    check_collision,
};


pub struct Feature;

impl Feature {
    pub fn new(planets: &mut store::Strong<Planet>) -> Self {
        planets.insert(Planet {
            pos:  Pnt2::new(0.0, 0.0),
            size: 100.0,
        });

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
