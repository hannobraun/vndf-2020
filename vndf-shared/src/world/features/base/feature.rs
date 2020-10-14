use rinnsal::EventBuf;

use super::{ComponentRemoved, Update};

pub struct Feature {
    pub component_removed: EventBuf<ComponentRemoved>,
    pub update: EventBuf<Update>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            component_removed: EventBuf::new(),
            update: EventBuf::new(),
        }
    }
}
