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


pub struct Systems<Bodies, Healths, Planets, Positions> {
    pub bodies:    Bodies,
    pub healths:   Healths,
    pub planets:   Planets,
    pub positions: Positions,
}

impl<B, H, Pl, Po> Systems<B, H, Pl, Po>
    where
        B:  store::Get<Body> + for<'r> store::ValuesMut<'r, Body>,
        H:  for<'r> store::ValuesMut<'r, Health>,
        Pl: for<'r> store::Values<'r, Planet>,
        Po: store::Get<Position>,
{
    pub fn on_update(&mut self) {
        self.apply_gravitation();
        self.check_collision();
    }

    pub fn apply_gravitation(&mut self) -> Option<()> {
        let bodies  = self.bodies.values_mut();
        let planets = self.planets.values();

        for (body, planet) in bodies.zip(planets) {
            planet.apply_gravitation(body, &self.positions);
        }

        Some(())
    }

    pub fn check_collision(&mut self) -> Option<()> {
        for health in self.healths.values_mut() {
            let body = self.bodies.get(&health.body)?;
            let pos  = self.positions.get(&body.pos)?;

            for planet in self.planets.values() {
                if pos.0.distance(planet.pos) <= planet.size {
                    health.value = 0.0;
                }
            }
        }

        Some(())
    }
}
