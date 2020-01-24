use crate::{
    cgs::Store,
    game::features::{
        crafts::components::Craft,
        physics::components::Body,
        players::PlayerId,
        health::components::Health,
    },
    world,
};

use super::components::Ship;


pub struct ShipEntity {
    pub owner: PlayerId,
    pub color: [f32; 3],
}

impl ShipEntity {
    pub fn create(&self, world: &mut world::Spawn, ships: &mut Store<Ship>) {
        let craft = Craft {
            engine_on: false,
            thrust:    100.0,
            fuel:      1200.0,
            owner:     self.owner,
        };

        let entity = world.spawn((Body::new(), craft, Health::new(10.0)));
        ships.insert(Ship::new(entity, self.color));
    }
}
