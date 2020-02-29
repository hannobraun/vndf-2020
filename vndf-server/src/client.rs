use std::{
    collections::HashMap,
    time::{
        Duration,
        Instant,
    },
};

use vndf_shared::{
    data::{
        ClientData,
        ClientHandle,
    },
    game::base::{
        Component,
        ComponentHandle,
    },
};


pub struct Client {
    data:    ClientData,
    updates: HashMap<ClientHandle, Instant>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            data:    ClientData::new(),
            updates: HashMap::new(),
        }
    }

    pub fn remove(&mut self, handle: &ClientHandle) {
        self.updates.remove(handle);
        handle.remove(&mut self.data);
    }

    pub fn update(&mut self, component: Component) -> bool {
        let handle = ComponentHandle::from_component(&component);
        let handle: ClientHandle = handle.into();

        let recently_updated = self.updates
            .get(&handle)
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
            self.updates.insert(handle, Instant::now());
        }

        should_update
    }
}