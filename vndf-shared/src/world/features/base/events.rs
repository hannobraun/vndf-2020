use crate::{data, world::math::Scalar};

pub struct ComponentRemoved {
    pub handle: data::client::Handle,
}

pub struct Update {
    pub dt: Scalar,
}
