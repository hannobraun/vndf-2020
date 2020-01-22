use crate::cgs::Handle;


pub struct EntityRemoved {
    pub handle: hecs::Entity,
}

pub struct ItemRemoved {
    pub handle: Handle,
}

pub struct Update {
    pub dt: f32,
}
