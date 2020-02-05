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
        missiles::{
            Missile,
            Target,
        },
        physics::{
            Body,
            Position,
            Velocity,
        },
        ships::Ship,
    },
};


macro_rules! components {
    ($($component:ident,)*) => {
        #[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq)]
        pub enum Component {
            $($component($component),)*
        }

        #[derive(
            Clone, Copy, Debug, Deserialize, Serialize, Eq, PartialEq, Hash
        )]
        pub enum ComponentHandle {
            $($component(Handle),)*
        }

        impl ComponentHandle {
            pub fn from_handle(handle: Handle, component: &Component) -> Self {
                match component {
                    $(
                        Component::$component(_) =>
                            ComponentHandle::$component(handle),
                    )*
                }
            }
        }
    };
}

components!(
    Body,
    Craft,
    Explosion,
    Health,
    Missile,
    Position,
    Ship,
    Target,
    Velocity,
);
