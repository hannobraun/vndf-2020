use std::{
    collections::VecDeque,
    time::{
        Duration,
        Instant,
    },
};

use crate::shared::{
    cgs::Handle,
    game::{
        Diagnostics,
        base::{
            Component,
            ComponentHandle,
        },
        players::PlayerId,
    },
    net::data::Data,
};


pub struct State {
    pub own_id:      Option<PlayerId>,
    pub diagnostics: Option<Diagnostics>,
    pub statistics:  Statistics,
    pub data:        Data,
}

impl State {
    pub fn new() -> Self {
        Self {
            own_id:      None,
            diagnostics: None,
            statistics:  Statistics::new(),
            data:        Data::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.statistics.update();

        for body in self.data.bodies.values_mut() {
            body.update(
                dt,
                &mut self.data.positions,
                &mut self.data.velocities,
            );
        }
        for explosion in self.data.explosions.values_mut() {
            explosion.update(dt);
        }
    }

    pub fn update_component(&mut self, handle: Handle, component: Component) {
        self.statistics.updates.push_back(Instant::now());
        self.data.update(handle, component);
    }

    pub fn remove_component(&mut self, handle: ComponentHandle) {
        self.statistics.removals.push_back(Instant::now());
        self.data.remove(handle);
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
