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
            Velocity,
        },
        ships::Ship,
    },
};


#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum Component {
    Body(Body),
    Craft(Craft),
    Explosion(Explosion),
    Health(Health),
    Missile(Missile),
    Position(Position),
    Ship(Ship),
    Velocity(Velocity),
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
pub enum ComponentHandle {
    Body(Handle),
    Craft(Handle),
    Explosion(Handle),
    Health(Handle),
    Missile(Handle),
    Position(Handle),
    Ship(Handle),
    Velocity(Handle),
}
