use toadster::StrongStore;

use super::{
    Body,
    Direction,
    Position,
    Velocity,
};


pub fn update_bodies(
    bodies:     &mut StrongStore<Body>,
    directions: &mut StrongStore<Direction>,
    positions:  &mut StrongStore<Position>,
    velocities: &mut StrongStore<Velocity>,
    world_size: f32,
    dt:         f32,
) {
    for body in bodies.values_mut() {
        body.update(dt, directions, positions, velocities);
        body.enforce_boundary(world_size, positions, velocities);
    }
}
