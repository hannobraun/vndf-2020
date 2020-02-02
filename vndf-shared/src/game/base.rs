pub mod events;
pub mod feature;


pub use self::{
    events::*,
    feature::*,
};


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
        physics::{
            Body,
            Position,
        },
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
    Position(Position),
    Ship(Ship),
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum ComponentHandle {
    Body(Handle),
    Craft(Handle),
    Explosion(Handle),
    Health(Handle),
    Missile(Handle),
    Position(Handle),
    Ship(Handle),
}
