use crate::data::ClientHandle;


pub struct ComponentRemoved {
    pub handle: ClientHandle,
}

pub struct Update {
    pub dt: f32,
}
