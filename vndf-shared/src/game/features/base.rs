use crate::game::ItemHandle;


pub struct EntityRemoved {
    pub handle: hecs::Entity,
}

pub struct ItemRemoved {
    pub handle: ItemHandle,
}

pub struct Update {
    pub dt: f32,
}
