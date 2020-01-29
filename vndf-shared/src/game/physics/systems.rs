use crate::cgs::Store;

use super::Body;


pub fn update_bodies(bodies: &mut Store<Body>, world_size: f32, dt: f32) {
    for body in bodies.values_mut() {
        body.update(dt);
        body.enforce_boundary(world_size);
    }
}
