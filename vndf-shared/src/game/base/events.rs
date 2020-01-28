use super::ComponentHandle;


pub struct ComponentRemoved {
    pub handle: ComponentHandle,
}

pub struct Update {
    pub dt: f32,
}
