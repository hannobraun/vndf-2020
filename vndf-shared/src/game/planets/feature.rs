use toadster::store;

use crate::{
    game::{
        health::Health,
        physics::components::{
            Body,
            Position,
        },
    },
    math::prelude::*,
};

use super::Planet;


pub struct Feature<'r> {
    pub bodies:    &'r mut store::Strong<Body>,
    pub healths:   &'r mut store::Strong<Health>,
    pub planets:   &'r store::Strong<Planet>,
    pub positions: &'r store::Strong<Position>,
}

impl Feature<'_> {
    pub fn on_update(&mut self) {
        self.apply_gravitation();
        self.check_collision();
    }

    pub fn apply_gravitation(&mut self) -> Option<()> {
        let bodies  = self.bodies.values_mut();
        let planets = self.planets.values();

        for (body, planet) in bodies.zip(planets) {
            planet.apply_gravitation(body, self.positions);
        }

        Some(())
    }

    pub fn check_collision(&mut self) -> Option<()> {
        for planet in self.planets.values() {
            for health in self.healths.values_mut() {
                let body = self.bodies.get(&health.body)?;
                let pos  = self.positions.get(&body.pos)?;

                if pos.0.distance(planet.pos) <= planet.size {
                    health.value = 0.0;
                }
            }
        }

        Some(())
    }
}
