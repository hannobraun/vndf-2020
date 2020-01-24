use std::collections::HashMap;

use hecs::World;

use crate::shared::{
    cgs::{
        Handle,
        SecondaryStore,
    },
    game::{
        Item,
        ItemHandle,
        features::{
            players::PlayerId,
            ships::components::Ship,
        },
    },
    net::game::{
        Entity,
        Id,
    },
};


pub struct State {
    pub world:  World,
    pub own_id: Option<PlayerId>,

    ids: HashMap<Id, hecs::Entity>,

    pub ships: SecondaryStore<Ship>,
}

impl State {
    pub fn new() -> Self {
        Self {
            world:  World::new(),
            own_id: None,
            ids:    HashMap::new(),

            ships: SecondaryStore::new(),
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
        if let Some(hecs_entity) = self.ids.remove(&id) {
            self.world.despawn(hecs_entity)
                .expect("Entity did not exist, but id was being tracked");
        }
        else {
            // The entity might not exist, if we logged in right after the
            // entity was removed. Nothing to do in that case.
        }
    }

    pub fn update_item(&mut self, handle: Handle, item: Item) {
        match item {
            Item::Ship(ship) => {
                self.ships.insert(handle, ship);
            }
        }
    }

    pub fn remove_item(&mut self, handle: ItemHandle) {
        match handle {
            ItemHandle::Ship(handle) => {
                self.ships.remove(handle);
            }
        }
    }
}
