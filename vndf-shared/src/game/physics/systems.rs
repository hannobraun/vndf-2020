use toadster::store;

use crate::game::planets::{
    Planet,
    Planets,
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
        world_size: f32,
        dt:         f32,
) {
    for body in bodies.values_mut() {
        body.update(dt, Planets(planets), &mut positions, &mut velocities);
        body.enforce_boundary(world_size, positions, velocities);
    }
}
