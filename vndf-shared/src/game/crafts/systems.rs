use crate::{
    cgs::Store,
    game::physics::Body,
};

use super::Craft;


pub fn update_bodies(bodies: &mut Store<Body>, world_size: f32, dt: f32) {
    for body in bodies.values_mut() {
        body.update(dt);
        body.enforce_boundary(world_size);
    }
}

pub fn update_crafts(
    bodies: &mut Store<Body>,
    crafts: &mut Store<Craft>,
    dt:     f32,
) {
    for craft in crafts.values_mut() {
        if let Some(mut body) = bodies.get_mut(craft.body) {
            craft.apply_thrust(&mut body, dt);
        }
    }
}
