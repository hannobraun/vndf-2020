use toadster::store;

use crate::world::{crafts::Craft, physics::Body, Scalar};

use super::Ship;

pub fn update_ships(
    dt: Scalar,
    bodies: &mut store::Strong<Body>,
    crafts: &store::Strong<Craft>,
    ships: &mut store::Strong<Ship>,
) {
    for ship in ships.values_mut() {
        ship.update(dt, bodies, crafts);
    }
}
