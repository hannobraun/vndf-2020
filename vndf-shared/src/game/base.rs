use serde::{
    Deserialize,
    Serialize,
};

use crate::game::{
    ItemHandle,
    explosions::Explosion,
    ships::Ship,
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Component {
    Explosion(Explosion),
    Ship(Ship),
}


pub struct EntityRemoved {
    pub handle: hecs::Entity,
}

pub struct ItemRemoved {
    pub handle: ItemHandle,
}

pub struct Update {
    pub dt: f32,
}
