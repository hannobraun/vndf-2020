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
    net::data::Data,
};


pub struct Client {
    data:    Data,
    updates: HashMap<ComponentHandle, Instant>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            data:    Data::new(),
            updates: HashMap::new(),
        }
    }

    pub fn remove(&mut self, handle: ComponentHandle) {
        self.data.remove(handle);
        self.updates.remove(&handle);
    }

    pub fn update(&mut self, component: Component) -> bool {
        let component_handle = ComponentHandle::from_component(&component);

        let recently_updated = self.updates
            .get(&component_handle)
            .map(|last_update|
                last_update.elapsed() < Duration::from_secs(1)
            )
            .unwrap_or(false);

        let data_changed = self.data.update(component);

        use Component::*;
        let should_update = match component {
            // These components are interpolated client-side.
            Direction(_, _)
                | Position(_, _)
                | Velocity(_, _)
                | Explosion(_, _)
                | Fuel(_, _)
            =>
            {
                data_changed && !recently_updated
            }
            _ => {
                data_changed
            }
        };

        if should_update {
            self.updates.insert(component_handle, Instant::now());
        }

        should_update
    }
}