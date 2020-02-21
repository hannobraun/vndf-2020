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


pub struct Feature {
    pub planets: store::Strong<Planet>,
}

impl Feature {
    pub fn new() -> Self {
        let mut planets = store::Strong::new();

        planets.insert(Planet {
            pos:  Pnt2::new(0.0, 0.0),
            size: 100.0,
        });

        Self {
            planets,
        }
    }

    pub fn on_update(&self,
        bodies:    &mut store::Strong<Body>,
        healths:   &mut store::Strong<Health>,
        positions: &store::Strong<Position>,
    ) {
        apply_gravitation(
            bodies,
            &self.planets,
            positions,
        );
        check_collision(
            bodies,
            healths,
            &self.planets,
            positions,
        );
    }
}
