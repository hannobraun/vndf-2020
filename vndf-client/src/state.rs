use std::collections::HashMap;

use hecs::World;

use crate::shared::net::game::{
    Entity,
    Id,
};


pub struct State {
    pub world: World,

    ids: HashMap<Id, hecs::Entity>,
}

impl State {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            ids:   HashMap::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        let hecs_entity = entity.spawn(&mut self.world);
        self.ids.insert(entity.id, hecs_entity);
    }

    pub fn update_entity(&mut self, entity: Entity) {
        let hecs_entity = self.ids.get(&entity.id)
            .expect("Server sent update for unknown entity");
        entity.update(*hecs_entity, &mut self.world)
            .expect("Entity did not exist, but id was being tracked");
    }

    pub fn remove_entity(&mut self, id: Id) {
        let hecs_entity = self.ids.remove(&id)
            .expect("Server sent removal message for unknown entity");
        self.world.despawn(hecs_entity)
            .expect("Entity did not exist, but id was being tracked");
    }
}
