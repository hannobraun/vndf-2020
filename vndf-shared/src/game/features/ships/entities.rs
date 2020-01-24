use crate::{
    cgs::Store,
    game::{
        entities,
        features::players::PlayerId,
    },
    world,
};

use super::components::Ship;


pub struct ShipEntity {
    pub player_id: PlayerId,
    pub color:     [f32; 3],
}

impl ShipEntity {
    pub fn create(&self, world: &mut world::Spawn, ships: &mut Store<Ship>) {
        let entity = world.spawn(entities::ship(self.player_id));
        ships.insert(Ship::new(entity, self.color));
    }
}
