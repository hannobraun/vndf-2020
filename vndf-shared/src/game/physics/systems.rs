use toadster::Store;

use super::{
    Body,
    Direction,
    Position,
    Velocity,
};


pub fn update_bodies(
    bodies:     &mut Store<Body>,
    directions: &mut Store<Direction>,
    positions:  &mut Store<Position>,
    velocities: &mut Store<Velocity>,
    world_size: f32,
    dt:         f32,
) {
    for body in bodies.values_mut() {
        body.update(dt, directions, positions, velocities);
        body.enforce_boundary(world_size, positions, velocities);
    }
}
