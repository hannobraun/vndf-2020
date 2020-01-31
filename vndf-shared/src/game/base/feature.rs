use crate::events;

use super::{
    ComponentRemoved,
    Update,
};


pub struct Feature {
    pub component_removed: events::Buf<ComponentRemoved>,
    pub update:            events::Buf<Update>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            component_removed: events::Buf::new(),
            update:            events::Buf::new(),
        }
    }
}
