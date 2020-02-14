use super::{
    ComponentRemoved,
    Update,
};


pub struct Feature {
    pub component_removed: bach::Buf<ComponentRemoved>,
    pub update:            bach::Buf<Update>,
}

impl Feature {
    pub fn new() -> Self {
        Self {
            component_removed: bach::Buf::new(),
            update:            bach::Buf::new(),
        }
    }
}
