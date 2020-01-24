use super::ComponentHandle;


pub struct EntityRemoved {
    pub handle: hecs::Entity,
}

pub struct ItemRemoved {
    pub handle: ComponentHandle,
}

pub struct Update {
    pub dt: f32,
}
