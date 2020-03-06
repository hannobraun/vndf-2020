use std::{
    collections::HashMap,
    time::{
        Duration,
        Instant,
    },
};

use vndf_shared::data;


pub struct Client {
    data:    data::client::Components,
    updates: HashMap<data::client::Handle, Instant>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            data:    data::client::Components::new(),
            updates: HashMap::new(),
        }
    }

    pub fn remove(&mut self, handle: &data::client::Handle) {
        self.updates.remove(handle);
        handle.remove(&mut self.data);
    }

    pub fn update(&mut self, component: data::client::Component) -> bool {
        let handle = data::client::Handle::from_component(&component);

        let recently_updated = self.updates
            .get(&handle)
            .map(|last_update|
                last_update.elapsed() < Duration::from_secs(1)
            )
            .unwrap_or(false);

        use data::client::Component::*;
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

        let data_changed = component.update(&mut self.data);

        let should_update = if is_interpolated {
            data_changed && !recently_updated
        }
        else {
            data_changed
        };

        if should_update {
            self.updates.insert(handle, Instant::now());
        }

        should_update
    }
}