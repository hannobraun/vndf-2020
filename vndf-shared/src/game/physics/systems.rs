use toadster::store;

use super::{
    Body,
    Direction,
    Position,
    Velocity,
};


pub fn update_bodies(
    bodies:     &mut store::Strong<Body>,
    directions: &mut store::Strong<Direction>,
    positions:  &mut store::Strong<Position>,
    velocities: &mut store::Strong<Velocity>,
    world_size: f32,
    dt:         f32,
) {
    for body in bodies.values_mut() {
        body.update(dt, directions, positions, velocities);
        body.enforce_boundary(world_size, positions, velocities);
    }
}
