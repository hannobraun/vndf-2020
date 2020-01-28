use crate::{
    cgs::Store,
    world,
};

use super::Ship;


pub fn update_ships(
    ships: &mut Store<Ship>,
    world: &mut world::Query,
) {
    for ship in ships.values() {
        ship.update(world);
    }
}
