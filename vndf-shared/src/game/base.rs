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
        missiles::Missile,
        physics::Body,
        ships::Ship,
    },
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Component {
    Body(Body),
    Explosion(Explosion),
    Missile(Missile),
    Ship(Ship),
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum ComponentHandle {
    Body(Handle),
    Explosion(Handle),
    Missile(Handle),
    Ship(Handle),
}
