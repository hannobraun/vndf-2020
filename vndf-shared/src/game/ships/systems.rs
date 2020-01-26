use crate::{
    cgs::Store,
    game::physics::Body,
    world,
};

use super::Ship;


pub fn update_ships(
    ships: &mut Store<Ship>,
    world: world::Query,
) {
    for ship in ships.values() {
        let body = world.get_mut::<Body>(hecs::Entity::from_bits(ship.entity));

        if let Ok(mut body) = body {
            ship.update(&mut body);
        }
    }
}
