use toadster::Store;

use crate::game::{
    crafts::Craft,
    physics::Body,
};

use super::Ship;


pub fn update_ships(
    bodies: &mut Store<Body>,
    crafts: &Store<Craft>,
    ships:  &mut Store<Ship>,
) {
    for ship in ships.values() {
        ship.update(bodies, crafts);
    }
}
