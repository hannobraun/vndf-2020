use toadster::store;

use crate::{
    game::physics::{
        Body,
        Direction,
    },
};

use super::{
    Craft,
    Fuel,
};


pub fn update_crafts(
    bodies:     &mut store::Strong<Body>,
    crafts:     &mut store::Strong<Craft>,
    directions: &store::Strong<Direction>,
    fuels:      &mut store::Strong<Fuel>,
    dt:         f32,
) {
    for craft in crafts.values_mut() {
        craft.apply_thrust(dt, bodies, directions, fuels);
    }
}
