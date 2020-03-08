use toadster::store;

use crate::{
    game::{
        health::Health,
        physics::{
            Body,
            Position,
        },
    },
    math::prelude::*,
};

use super::Planet;


pub fn apply_gravitation(
    bodies:    &mut store::Strong<Body>,
    planets:   &store::Strong<Planet>,
    positions: &impl store::Get<Position>,
)
    -> Option<()>
{
    for planet in planets.values() {
        for body in bodies.values_mut() {
            planet.apply_gravitation(body, positions);
        }
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
