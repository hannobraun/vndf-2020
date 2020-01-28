use super::ComponentHandle;


pub struct ItemRemoved {
    pub handle: ComponentHandle,
}

pub struct Update {
    pub dt: f32,
}
