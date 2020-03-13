use toadster::store;

use crate::{
    game::{
        health::Health,
        physics::components::{
            Body,
            Position,
        },
    },
    math::{
        prelude::*,
        Pnt2,
    },
};

use super::Planet;


pub struct Systems<B, H, Pl, Po> {
    pub bodies:    B,
    pub healths:   H,
    pub planets:   Planets<Pl>,
    pub positions: Po,
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
        self.check_collisions();
    }

    pub fn apply_gravitation(&mut self) -> Option<()> {
        for body in self.bodies.values_mut() {
            self.planets.apply_gravitation(body, &self.positions);
        }

        Some(())
    }

    pub fn check_collisions(&mut self) -> Option<()> {
        for health in self.healths.values_mut() {
            let body = self.bodies.get(&health.body)?;
            let pos  = self.positions.get(&body.pos)?;

            if self.planets.check_collision(pos.0) {
                health.value = 0.0;
            }
        }

        Some(())
    }
}


pub struct Planets<S>(pub S);

impl<S> Planets<S>
    where S: for<'r> store::Values<'r, Planet>
{
    pub fn apply_gravitation<P>(&self, body: &mut Body, positions: P)
        where P: store::Get<Position>
    {
        for planet in self.0.values() {
            planet.apply_gravitation(body, &positions);
        }
    }

    pub fn check_collision(&self, pos: Pnt2) -> bool {
        for planet in self.0.values() {
            if pos.distance(planet.pos) <= planet.size {
                return true;
            }
        }

        false
    }
}
