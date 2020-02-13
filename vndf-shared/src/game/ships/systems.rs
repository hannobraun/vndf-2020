use toadster::StrongStore;

use crate::game::{
    crafts::Craft,
    physics::Body,
};

use super::Ship;


pub fn update_ships(
    bodies: &mut StrongStore<Body>,
    crafts: &StrongStore<Craft>,
    ships:  &mut StrongStore<Ship>,
) {
    for ship in ships.values() {
        ship.update(bodies, crafts);
    }
}
