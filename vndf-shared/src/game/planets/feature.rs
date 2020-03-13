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
        Self::apply_gravitation(
            self.bodies,
            self.planets,
            self.positions,
        );
        Self::check_collision(
            self.bodies,
            self.healths,
            self.planets,
            self.positions,
        );
    }

    pub fn apply_gravitation(
        bodies:    &mut impl for<'r> store::ValuesMut<'r, Body>,
        planets:   &impl for<'r> store::Values<'r, Planet>,
        positions: impl store::Get<Position>,
    )
        -> Option<()>
    {
        let bodies  = bodies.values_mut();
        let planets = planets.values();

        for (body, planet) in bodies.zip(planets) {
            planet.apply_gravitation(body, &positions);
        }

        Some(())
    }

    pub fn check_collision(
        bodies:    &store::Strong<Body>,
        healths:   &mut store::Strong<Health>,
        planets:   &store::Strong<Planet>,
        positions: &store::Strong<Position>,
    )
        -> Option<()>
    {
        for planet in planets.values() {
            for health in healths.values_mut() {
                let body = bodies.get(&health.body)?;
                let pos  = positions.get(&body.pos)?;

                if pos.0.distance(planet.pos) <= planet.size {
                    health.value = 0.0;
                }
            }
        }

        Some(())
    }
}
