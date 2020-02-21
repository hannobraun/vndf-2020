use toadster::store;

use crate::{
    game::physics::components::{
        Body,
        Position,
    },
    math::Pnt2,
};

use super::{
    Planet,
    apply_gravitation,
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
        positions: &store::Strong<Position>,
    ) {
        apply_gravitation(
            bodies,
            &self.planets,
            positions,
        );
    }
}
