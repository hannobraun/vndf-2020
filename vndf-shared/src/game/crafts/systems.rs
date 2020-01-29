use crate::{
    cgs::Store,
    game::physics::Body,
};

use super::Craft;


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
