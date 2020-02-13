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
        crafts::{
            Craft,
            Fuel,
        },
        explosions::Explosion,
        health::Health,
        loot::Loot,
        missiles::{
            Missile,
            Target,
        },
        physics::{
            Body,
            Direction,
            Position,
            Velocity,
        },
        ships::Ship,
    },
};


macro_rules! components {
    ($($component:ident,)*) => {
        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
        pub enum Component {
            $($component(Handle<$component>, $component),)*
        }

        #[derive(
            Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash
        )]
        pub enum ComponentHandle {
            $($component(Handle<$component>),)*
        }

        impl ComponentHandle {
            pub fn from_component(component: &Component) -> Self {
                match component {
                    $(
                        Component::$component(handle, _) =>
                            ComponentHandle::$component(*handle),
                    )*
                }
            }
        }
    };
}

components!(
    Body,
    Craft,
    Direction,
    Explosion,
    Fuel,
    Health,
    Loot,
    Missile,
    Position,
    Ship,
    Target,
    Velocity,
);
