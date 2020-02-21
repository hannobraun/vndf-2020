use std::f32::consts::PI;

use toadster::store;

use crate::{
    game::physics::{
        Body,
        Position,
    },
    math::prelude::*,
};

use super::Planet;


pub fn apply_gravitation(
    bodies:    &mut store::Strong<Body>,
    planets:   &store::Strong<Planet>,
    positions: &store::Strong<Position>,
)
    -> Option<()>
{
    for planet in planets.values() {
        for body in bodies.values_mut() {
            let pos = positions.get(&body.pos)?;

            // The gravitational constant of our universe. Completely made up.
            const G: f32 = 5.0;

            let dist = pos.0.distance(planet.pos);
            let mass = PI * planet.size.powi(2);
            let acc  = G * mass / dist.powi(2);

            let acc = (planet.pos - pos.0).normalize() * acc;
            body.acc += acc;
        }
    }

    Some(())
}
