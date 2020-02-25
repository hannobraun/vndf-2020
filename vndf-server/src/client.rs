use std::{
    collections::HashMap,
    time::{
        Duration,
        Instant,
    },
};

use vndf_shared::{
    game::base::{
        Component,
        ComponentHandle,
    },
    net::data::ClientData,
};


pub struct Client {
    data:    ClientData,
    updates: HashMap<ComponentHandle, Instant>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            data:    ClientData::new(),
            updates: HashMap::new(),
        }
    }

    pub fn remove(&mut self, handle: &ComponentHandle) {
        self.data.remove(handle);
        self.updates.remove(handle);
    }

    pub fn update(&mut self, component: Component) -> bool {
        let component_handle = ComponentHandle::from_component(&component);

        let recently_updated = self.updates
            .get(&component_handle)
            .map(|last_update|
                last_update.elapsed() < Duration::from_secs(1)
            )
            .unwrap_or(false);

        use Component::*;
        let is_interpolated = match component {
            // These components are interpolated client-side.
            Direction(_, _)
                | Position(_, _)
                | Velocity(_, _)
                | Explosion(_, _)
                | Fuel(_, _)
            =>
                true,
            _ =>
                false,
        };

        let data_changed = self.data.update(component);

        let should_update = if is_interpolated {
            data_changed && !recently_updated
        }
        else {
            data_changed
        };

        if should_update {
            self.updates.insert(component_handle, Instant::now());
        }

        should_update
    }
}