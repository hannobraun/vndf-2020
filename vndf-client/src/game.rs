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
        health::Health,
        missiles::Missile,
        physics::Body,
        players::PlayerId,
        ships::Ship,
    },
};


pub struct State {
    pub world:  World,
    pub own_id: Option<PlayerId>,

    pub bodies:     SecondaryStore<Body>,
    pub crafts:     SecondaryStore<Craft>,
    pub explosions: SecondaryStore<Explosion>,
    pub healths:    SecondaryStore<Health>,
    pub missiles:   SecondaryStore<Missile>,
    pub ships:      SecondaryStore<Ship>,
}

impl State {
    pub fn new() -> Self {
        Self {
            world:  World::new(),
            own_id: None,

            bodies:     SecondaryStore::new(),
            crafts:     SecondaryStore::new(),
            explosions: SecondaryStore::new(),
            healths:    SecondaryStore::new(),
            missiles:   SecondaryStore::new(),
            ships:      SecondaryStore::new(),
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
            Component::Health(health) => {
                self.healths.insert(handle, health);
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
            ComponentHandle::Health(handle) => {
                self.healths.remove(handle);
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
