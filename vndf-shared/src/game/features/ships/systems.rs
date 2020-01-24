use crate::{
    cgs::Store,
    game::features::{
        physics::components::Body,
        ships::components::Ship,
    },
    world,
};


pub fn update_ships(
    ships: &mut Store<Ship>,
    world: world::Query,
) {
    for ship in ships.values() {
        let mut body = world
            .get_mut::<Body>(hecs::Entity::from_bits(ship.entity))
            .expect("Failed to get body for ship");
        ship.update(&mut body);
    }
}
