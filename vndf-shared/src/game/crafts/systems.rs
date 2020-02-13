use toadster::StrongStore;

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
    bodies:     &mut StrongStore<Body>,
    crafts:     &mut StrongStore<Craft>,
    directions: &StrongStore<Direction>,
    fuels:      &mut StrongStore<Fuel>,
    dt:         f32,
) {
    for craft in crafts.values_mut() {
        craft.apply_thrust(dt, bodies, directions, fuels);
    }
}
