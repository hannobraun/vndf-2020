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
        Vec2,
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
        self.check_collisions();

        // You might expect code here that simulates the gravitational pull of
        // the planets and updates all bodies accordingly. This can't be handled
        // here though, unless a very basic numerical integration method is used
        // for the physics simulation, like an Euler variant.
        //
        // More advanced numerical integration methods need to sample the
        // acceleration acting on a body at multiple points, not just at the
        // frame boundary. This means, we can't just compute it here once and be
        // done with it.
        //
        // Acceleration due to gravitational pull is applied in the core physics
        // update code, where the numerical integration is handled.
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
    pub fn gravitation_at(&self, pos: Pnt2) -> Vec2 {
        let mut acc = Vec2::zero();

        for planet in self.0.values() {
            acc += planet.gravitation_at(pos);
        }

        acc
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
