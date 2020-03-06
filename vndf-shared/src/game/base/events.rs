use crate::data;


pub struct ComponentRemoved {
    pub handle: data::client::Handle,
}

pub struct Update {
    pub dt: f32,
}
