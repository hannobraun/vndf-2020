use toadster::store;

use crate::world::{math::Scalar, physics::Body};

use super::{Craft, Fuel};

pub fn update_crafts(
    bodies: &mut store::Strong<Body>,
    crafts: &mut store::Strong<Craft>,
    fuels: &mut store::Strong<Fuel>,
    dt: Scalar,
) {
    for craft in crafts.values_mut() {
        craft.apply_thrust(dt, bodies, fuels);
    }
}
