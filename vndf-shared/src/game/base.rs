pub mod events;


pub use self::events::*;


use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    cgs::Handle,
    game::{
        explosions::Explosion,
        ships::Ship,
    },
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Component {
    Explosion(Explosion),
    Ship(Ship),
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum ItemHandle {
    Explosion(Handle),
    Ship(Handle),
}
