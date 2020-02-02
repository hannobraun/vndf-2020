use crate::cgs::Store;

use super::{
    Body,
    Position,
};


pub fn update_bodies(
    bodies:     &mut Store<Body>,
    positions:  &mut Store<Position>,
    world_size: f32,
    dt:         f32,
) {
    for body in bodies.values_mut() {
        body.update(dt, positions);
        body.enforce_boundary(world_size, positions);
    }
}
