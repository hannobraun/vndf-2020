use std::{
    collections::VecDeque,
    time::{
        Duration,
        Instant,
    },
};


use crate::shared::{
    cgs::{
        Handle,
        SecondaryStore,
    },
    game::{
        Diagnostics,
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
        players::PlayerId,
        ships::Ship,
    },
};


pub struct State {
    pub own_id:      Option<PlayerId>,
    pub diagnostics: Option<Diagnostics>,
    pub statistics:  Statistics,

    pub bodies:     SecondaryStore<Body>,
    pub crafts:     SecondaryStore<Craft>,
    pub explosions: SecondaryStore<Explosion>,
    pub healths:    SecondaryStore<Health>,
    pub missiles:   SecondaryStore<Missile>,
    pub positions:  SecondaryStore<Position>,
    pub ships:      SecondaryStore<Ship>,
    pub velocities: SecondaryStore<Velocity>,
}

impl State {
    pub fn new() -> Self {
        Self {
            own_id:      None,
            diagnostics: None,
            statistics:  Statistics::new(),

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

    pub fn update(&mut self) {
        self.statistics.update();
    }

    pub fn update_component(&mut self, handle: Handle, component: Component) {
        self.statistics.updates.push_back(Instant::now());

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
        }
    }

    pub fn remove_component(&mut self, handle: ComponentHandle) {
        self.statistics.removals.push_back(Instant::now());

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
        }
    }
}


pub struct Statistics {
    pub updates:  VecDeque<Instant>,
    pub removals: VecDeque<Instant>,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            updates:  VecDeque::new(),
            removals: VecDeque::new(),
        }
    }

    pub fn update(&mut self) {
        while let Some(instant) = self.updates.front() {
            if instant.elapsed() > Duration::from_secs(1) {
                self.updates.pop_front();
            }
            else {
                break;
            }
        }
        while let Some(instant) = self.removals.front() {
            if instant.elapsed() > Duration::from_secs(1) {
                self.removals.pop_front();
            }
            else {
                break;
            }
        }
    }
}
