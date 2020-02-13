use toadster::Store;

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
    bodies:     &mut Store<Body>,
    crafts:     &mut Store<Craft>,
    directions: &Store<Direction>,
    fuels:      &mut Store<Fuel>,
    dt:         f32,
) {
    for craft in crafts.values_mut() {
        craft.apply_thrust(dt, bodies, directions, fuels);
    }
}
