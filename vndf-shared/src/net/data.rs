use crate::{
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
        physics::{
            Body,
            Position,
            Velocity,
        },
        ships::Ship,
    },
};


pub struct Data {
    pub bodies:     SecondaryStore<Body>,
    pub crafts:     SecondaryStore<Craft>,
    pub explosions: SecondaryStore<Explosion>,
    pub healths:    SecondaryStore<Health>,
    pub missiles:   SecondaryStore<Missile>,
    pub positions:  SecondaryStore<Position>,
    pub ships:      SecondaryStore<Ship>,
    pub velocities: SecondaryStore<Velocity>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            bodies:     SecondaryStore::new(),
            crafts:     SecondaryStore::new(),
            explosions: SecondaryStore::new(),
            healths:    SecondaryStore::new(),
            missiles:   SecondaryStore::new(),
            positions:  SecondaryStore::new(),
            ships:      SecondaryStore::new(),
            velocities: SecondaryStore::new(),
        }
    }

    pub fn update(&mut self, handle: Handle, component: Component) {
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
            Component::Position(position) => {
                self.positions.insert(handle, position);
            }
            Component::Ship(ship) => {
                self.ships.insert(handle, ship);
            }
            Component::Velocity(velocity) => {
                self.velocities.insert(handle, velocity);
            }
        }
    }

    pub fn remove(&mut self, handle: ComponentHandle) {
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
            ComponentHandle::Position(handle) => {
                self.positions.remove(handle);
            }
            ComponentHandle::Ship(handle) => {
                self.ships.remove(handle);
            }
            ComponentHandle::Velocity(handle) => {
                self.velocities.remove(handle);
            }
        }
    }
}
