use std::collections::HashMap;

use hecs::World;

use crate::shared::{
    cgs::{
        Handle,
        SecondaryStore,
    },
    game::{
        base::{
            Component,
            ComponentHandle,
        },
        crafts::Craft,
        explosions::Explosion,
        missiles::Missile,
        physics::Body,
        players::PlayerId,
        ships::Ship,
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

    pub bodies:     SecondaryStore<Body>,
    pub crafts:     SecondaryStore<Craft>,
    pub explosions: SecondaryStore<Explosion>,
    pub missiles:   SecondaryStore<Missile>,
    pub ships:      SecondaryStore<Ship>,
}

impl State {
    pub fn new() -> Self {
        Self {
            world:  World::new(),
            own_id: None,
            ids:    HashMap::new(),

            bodies:     SecondaryStore::new(),
            crafts:     SecondaryStore::new(),
            explosions: SecondaryStore::new(),
            missiles:   SecondaryStore::new(),
            ships:      SecondaryStore::new(),
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

    pub fn update_component(&mut self, handle: Handle, component: Component) {
        match component {
            Component::Body(body) => {
                self.bodies.insert(handle, body);
            }
            Component::Craft(craft) => {
                self.crafts.insert(handle, craft);
            }
            Component::Explosion(explosion) => {
                self.explosions.insert(handle, explosion);
            }
            Component::Missile(missile) => {
                self.missiles.insert(handle, missile);
            }
            Component::Ship(ship) => {
                self.ships.insert(handle, ship);
            }
        }
    }

    pub fn remove_component(&mut self, handle: ComponentHandle) {
        match handle {
            ComponentHandle::Body(handle) => {
                self.bodies.remove(handle);
            }
            ComponentHandle::Craft(handle) => {
                self.crafts.remove(handle);
            }
            ComponentHandle::Explosion(handle) => {
                self.explosions.remove(handle);
            }
            ComponentHandle::Missile(handle) => {
                self.missiles.remove(handle);
            }
            ComponentHandle::Ship(handle) => {
                self.ships.remove(handle);
            }
        }
    }
}
