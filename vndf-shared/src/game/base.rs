pub mod events;


pub use self::events::*;


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
