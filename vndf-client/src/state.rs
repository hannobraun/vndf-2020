use std::{
    collections::HashMap,
    net::SocketAddr,
};

use hecs::World;

use crate::shared::net::game::{
    Entity,
    Id,
};


pub struct State {
    pub world:  World,
    pub own_id: Option<SocketAddr>,

    ids: HashMap<Id, hecs::Entity>,
}

impl State {
    pub fn new() -> Self {
        Self {
            world:  World::new(),
            own_id: None,
            ids:    HashMap::new(),
        }
    }

    pub fn update_entity(&mut self, entity: Entity) {
        let hecs_entity: hecs::Entity = self.ids.get(&entity.id)
            .map(|hecs_entity| *hecs_entity)
            .unwrap_or_else(|| {
                let hecs_entity = entity.spawn(&mut self.world);
                self.ids.insert(entity.id, hecs_entity);
                hecs_entity
            });
        entity.update(hecs_entity, &mut self.world)
            .expect("Entity did not exist, but id was being tracked");
    }

    pub fn remove_entity(&mut self, id: Id) {
        let hecs_entity = self.ids.remove(&id)
            .expect("Server sent removal message for unknown entity");
        self.world.despawn(hecs_entity)
            .expect("Entity did not exist, but id was being tracked");
    }
}
