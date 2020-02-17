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
use toadster::{
    Handle,
    handle::{
        self,
        Untyped,
    },
};

use crate::{
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

        #[derive(Clone, Debug, Deserialize, Serialize, Eq, PartialEq, Hash)]
        pub enum ComponentHandle {
            $($component(Handle<$component>),)*
        }

        impl ComponentHandle {
            pub fn from_component(component: &Component) -> Self {
                match component {
                    $(
                        Component::$component(handle, _) =>
                            ComponentHandle::$component(handle.clone()),
                    )*
                }
            }

            pub fn is_strong(&self) -> bool {
                match self {
                    $(
                        Self::$component(handle) => handle.is_strong(),
                    )*
                }
            }

            pub fn as_weak(&self) -> Self {
                match self {
                    $(
                        Self::$component(handle) =>
                            Self::$component(handle.as_weak()),
                    )*
                }
            }

            pub fn into_weak(self) -> Self {
                match self {
                    $(
                        Self::$component(handle) =>
                            Self::$component(handle.into_weak()),
                    )*
                }
            }

            pub fn into_strong_untyped(self) -> handle::Strong<Untyped> {
                match self {
                    $(
                        Self::$component(handle) =>
                            handle.strong().into_untyped(),
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
