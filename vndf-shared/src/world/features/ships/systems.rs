use toadster::store;

use crate::world::{crafts::Craft, physics::Body};

use super::Ship;

pub fn update_ships(
    bodies: &mut store::Strong<Body>,
    crafts: &store::Strong<Craft>,
    ships: &mut store::Strong<Ship>,
) {
    for ship in ships.values() {
        ship.update(bodies, crafts);
    }
}
