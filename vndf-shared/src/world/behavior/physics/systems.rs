use toadster::store;

use crate::world::{
    math::Scalar,
    planets::{
        Planet,
        Planets,
    },
};

use super::{
    Body,
    Position,
    Velocity,
};


pub fn update_bodies(
        bodies:     &mut store::Strong<Body>,
        planets:    &store::Strong<Planet>,
    mut positions:  &mut store::Strong<Position>,
    mut velocities: &mut store::Strong<Velocity>,
        dt:         Scalar,
) {
    for body in bodies.values_mut() {
        body.update(dt, &Planets(planets), &mut positions, &mut velocities);
    }
}
