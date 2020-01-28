pub mod events;


pub use self::events::*;


use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    cgs::Handle,
    game::{
        crafts::Craft,
        explosions::Explosion,
        health::Health,
        missiles::Missile,
        physics::Body,
        ships::Ship,
    },
};


#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum Component {
    Body(Body),
    Craft(Craft),
    Explosion(Explosion),
    Health(Health),
    Missile(Missile),
    Ship(Ship),
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum ComponentHandle {
    Body(Handle),
    Craft(Handle),
    Explosion(Handle),
    Health(Handle),
    Missile(Handle),
    Ship(Handle),
}
